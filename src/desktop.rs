use futures_util::StreamExt;
use serde::de::DeserializeOwned;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use tauri::{plugin::PluginApi, AppHandle, Emitter, Manager, Runtime};

use crate::error::{Error, Result};
use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<Ffmpeg<R>> {
    Ok(Ffmpeg(app.clone()))
}

/// Access to the ffmpeg APIs.
pub struct Ffmpeg<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Ffmpeg<R> {
    /// 获取 FFmpeg 二进制文件的存储路径
    fn get_ffmpeg_dir(&self) -> Result<PathBuf> {
        let app_data_dir = self.0.path().app_data_dir().map_err(|e| {
            Error::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                e.to_string(),
            ))
        })?;

        let platform = self.get_platform()?;
        let ffmpeg_dir = app_data_dir.join("bin").join(platform);

        Ok(ffmpeg_dir)
    }

    /// 获取当前平台名称
    fn get_platform(&self) -> Result<&'static str> {
        #[cfg(target_os = "macos")]
        return Ok("macos");

        #[cfg(target_os = "windows")]
        return Ok("windows");

        #[cfg(target_os = "linux")]
        return Ok("linux");

        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
        return Err(Error::UnsupportedPlatform);
    }

    /// 获取默认下载配置
    fn get_default_config(&self) -> Result<DownloadConfig> {
        #[cfg(target_os = "macos")]
        return Ok(DownloadConfig {
            url: "https://evermeet.cx/ffmpeg/ffmpeg-8.0.zip".to_string(),
            executable_path: "ffmpeg".to_string(),
        });

        #[cfg(target_os = "windows")]
    return Ok(DownloadConfig {
      url: "https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-n8.0-latest-win64-gpl-8.0.zip".to_string(),
      executable_path: "bin/ffmpeg.exe".to_string(),
    });

        #[cfg(target_os = "linux")]
        return Ok(DownloadConfig {
            url: "https://johnvansickle.com/ffmpeg/releases/ffmpeg-release-amd64-static.tar.xz"
                .to_string(),
            executable_path: "ffmpeg".to_string(),
        });

        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
        return Err(Error::UnsupportedPlatform);
    }

    /// 获取 FFmpeg 可执行文件路径
    fn get_ffmpeg_executable_path(&self) -> Result<PathBuf> {
        let ffmpeg_dir = self.get_ffmpeg_dir()?;

        #[cfg(target_os = "windows")]
        let executable_name = "ffmpeg.exe";

        #[cfg(not(target_os = "windows"))]
        let executable_name = "ffmpeg";

        Ok(ffmpeg_dir.join(executable_name))
    }

    /// 检查 FFmpeg 是否可用
    pub fn check(&self) -> Result<CheckResponse> {
        let ffmpeg_path = self.get_ffmpeg_executable_path()?;

        if !ffmpeg_path.exists() {
            return Ok(CheckResponse {
                available: false,
                path: None,
                version: None,
            });
        }

        // 尝试执行 ffmpeg -version 获取版本信息
        let output = Command::new(&ffmpeg_path).arg("-version").output();

        match output {
            Ok(output) if output.status.success() => {
                let version_info = String::from_utf8_lossy(&output.stdout);
                let version = version_info.lines().next().map(|s| s.to_string());

                Ok(CheckResponse {
                    available: true,
                    path: Some(ffmpeg_path.to_string_lossy().to_string()),
                    version,
                })
            }
            _ => Ok(CheckResponse {
                available: false,
                path: Some(ffmpeg_path.to_string_lossy().to_string()),
                version: None,
            }),
        }
    }

    /// 下载 FFmpeg
    pub async fn download(&self, request: DownloadRequest) -> Result<DownloadResponse> {
        let config = request
            .config
            .unwrap_or_else(|| self.get_default_config().unwrap());

        let ffmpeg_dir = self.get_ffmpeg_dir()?;
        fs::create_dir_all(&ffmpeg_dir)?;

        // 下载文件
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(300))
            .build()?;

        let response = client.get(&config.url).send().await?;

        if !response.status().is_success() {
            return Err(Error::Download(format!(
                "Failed to download: HTTP {}",
                response.status()
            )));
        }

        let total_size = response.content_length();

        // 保存到临时文件
        let temp_file_path = ffmpeg_dir.join("ffmpeg_download.tmp");
        let mut file = fs::File::create(&temp_file_path)?;
        let mut stream = response.bytes_stream();
        let mut downloaded: u64 = 0;

        // 发送进度事件
        let app_handle = self.0.clone();

        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result?;
            file.write_all(&chunk)?;
            downloaded += chunk.len() as u64;

            // 计算进度并发送事件
            let progress = DownloadProgress {
                downloaded,
                total: total_size,
                percentage: total_size.map(|total| (downloaded as f64 / total as f64) * 100.0),
            };

            let _ = app_handle.emit("use-ffmpeg://download-progress", &progress);
        }

        drop(file);

        // 解压文件
        self.extract_archive(&temp_file_path, &ffmpeg_dir, &config.executable_path)?;

        // 删除临时文件
        fs::remove_file(&temp_file_path)?;

        let ffmpeg_path = self.get_ffmpeg_executable_path()?;

        // 在 Unix 系统上设置执行权限
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&ffmpeg_path)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&ffmpeg_path, perms)?;
        }

        Ok(DownloadResponse {
            success: true,
            path: Some(ffmpeg_path.to_string_lossy().to_string()),
            message: Some("FFmpeg downloaded successfully".to_string()),
        })
    }

    /// 解压归档文件
    fn extract_archive(
        &self,
        archive_path: &Path,
        target_dir: &Path,
        executable_path: &str,
    ) -> Result<()> {
        let file = fs::File::open(archive_path)?;
        let mut archive = zip::ZipArchive::new(file)?;

        // 查找可执行文件
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let file_path = file.name();

            // 检查是否是我们需要的可执行文件
            if file_path.ends_with(executable_path) || file_path.contains(executable_path) {
                let output_path = target_dir.join(
                    #[cfg(target_os = "windows")]
                    "ffmpeg.exe",
                    #[cfg(not(target_os = "windows"))]
                    "ffmpeg",
                );

                let mut outfile = fs::File::create(&output_path)?;
                std::io::copy(&mut file, &mut outfile)?;

                return Ok(());
            }
        }

        Err(Error::Extraction(format!(
            "Could not find executable at path: {}",
            executable_path
        )))
    }

    /// 执行 FFmpeg 命令
    pub fn execute(&self, request: ExecuteRequest) -> Result<ExecuteResponse> {
        let ffmpeg_path = self.get_ffmpeg_executable_path()?;

        if !ffmpeg_path.exists() {
            return Err(Error::FfmpegNotFound);
        }

        let output = Command::new(&ffmpeg_path)
            .args(&request.args)
            .output()
            .map_err(|e| Error::CommandExecution(e.to_string()))?;

        Ok(ExecuteResponse {
            success: output.status.success(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code(),
        })
    }

    /// 删除 FFmpeg
    pub fn remove(&self) -> Result<DeleteResponse> {
        let ffmpeg_dir = self.get_ffmpeg_dir()?;

        if !ffmpeg_dir.exists() {
            return Ok(DeleteResponse {
                success: true,
                message: Some("FFmpeg directory does not exist".to_string()),
            });
        }

        // 删除整个 FFmpeg 目录
        fs::remove_dir_all(&ffmpeg_dir)?;

        Ok(DeleteResponse {
            success: true,
            message: Some("FFmpeg deleted successfully".to_string()),
        })
    }
}
