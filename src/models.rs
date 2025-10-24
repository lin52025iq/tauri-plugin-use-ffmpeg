//! # Models
//!
//! 定义插件使用的数据结构和类型。

use serde::{Deserialize, Serialize};

/// FFmpeg 下载配置
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadConfig {
    /// 下载 URL
    pub url: String,
    /// 解压后 FFmpeg 可执行文件的相对路径
    pub executable_path: String,
}

/// 下载请求
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadRequest {
    /// 可选的下载配置，如果为 None 则使用默认配置
    pub config: Option<DownloadConfig>,
}

/// 下载响应
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadResponse {
    /// 是否成功
    pub success: bool,
    /// 下载后的文件路径
    pub path: Option<String>,
    /// 消息
    pub message: Option<String>,
}

/// 检查响应
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckResponse {
    /// FFmpeg 是否可用
    pub available: bool,
    /// FFmpeg 可执行文件路径
    pub path: Option<String>,
    /// FFmpeg 版本信息
    pub version: Option<String>,
}

/// 执行请求
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteRequest {
    /// FFmpeg 命令参数（不包含 ffmpeg 本身）
    pub args: Vec<String>,
}

/// 执行响应
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteResponse {
    /// 是否成功
    pub success: bool,
    /// 标准输出
    pub stdout: String,
    /// 标准错误输出
    pub stderr: String,
    /// 退出码
    pub exit_code: Option<i32>,
}

/// 下载进度
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadProgress {
    /// 已下载字节数
    pub downloaded: u64,
    /// 总字节数（如果已知）
    pub total: Option<u64>,
    /// 下载百分比（如果已知）
    pub percentage: Option<f64>,
}

/// 删除响应
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteResponse {
    /// 是否成功
    pub success: bool,
    /// 消息
    pub message: Option<String>,
}
