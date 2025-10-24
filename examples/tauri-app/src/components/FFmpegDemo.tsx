import { useState, useEffect } from "react";
import {
  check,
  download,
  execute,
  remove,
  type CheckResponse,
  type DownloadProgress,
} from "tauri-plugin-use-ffmpeg-api";
import { ask, message } from "@tauri-apps/plugin-dialog";
import "./FFmpegDemo.css";

function FFmpegDemo() {
  const [checkResult, setCheckResult] = useState<CheckResponse | null>(null);
  const [downloadProgress, setDownloadProgress] = useState<DownloadProgress | null>(null);
  const [isDownloading, setIsDownloading] = useState(false);
  const [executeResult, setExecuteResult] = useState("");
  const [executeError, setExecuteError] = useState("");

  useEffect(() => {
    checkFFmpeg();
  }, []);

  async function checkFFmpeg() {
    try {
      const result = await check();
      setCheckResult(result);
    } catch (error) {
      console.error("检查 FFmpeg 失败:", error);
    }
  }

  async function downloadFFmpeg() {
    try {
      setIsDownloading(true);
      setDownloadProgress(null);

      const result = await download(undefined, (progress) => {
        setDownloadProgress(progress);
      });

      if (result.success) {
        await checkFFmpeg();
        await message("FFmpeg 下载成功!", { title: "成功", kind: "info" });
      } else {
        await message("FFmpeg 下载失败: " + (result.message || "未知错误"), { title: "错误", kind: "error" });
      }
    } catch (error) {
      await message("下载失败: " + error, { title: "错误", kind: "error" });
    } finally {
      setIsDownloading(false);
      setDownloadProgress(null);
    }
  }

  async function runFFmpegVersion() {
    try {
      setExecuteResult("");
      setExecuteError("");

      const result = await execute(["-version"]);

      if (result.success) {
        setExecuteResult(result.stdout);
      } else {
        setExecuteError(result.stderr);
      }
    } catch (error) {
      setExecuteError(String(error));
    }
  }

  async function handleRemove() {
    const confirmed = await ask("确定要删除 FFmpeg 吗？", {
      title: "确认删除",
      kind: "warning",
    });

    if (!confirmed) {
      return;
    }

    try {
      const result = await remove();
      if (result.success) {
        await checkFFmpeg();
        await message("FFmpeg 删除成功!", { title: "成功", kind: "info" });
      } else {
        await message("FFmpeg 删除失败: " + (result.message || "未知错误"), { title: "错误", kind: "error" });
      }
    } catch (error) {
      await message("删除失败: " + error, { title: "错误", kind: "error" });
    }
  }

  return (
    <div className="ffmpeg-demo">
      <h2>FFmpeg 插件演示</h2>

      <div className="section">
        <h3>1. 检查 FFmpeg 状态</h3>
        <button onClick={checkFFmpeg}>检查 FFmpeg</button>

        {checkResult && (
          <div className="result">
            <p>
              <strong>可用:</strong> {checkResult.available ? "是" : "否"}
            </p>
            {checkResult.path && (
              <p>
                <strong>路径:</strong> {checkResult.path}
              </p>
            )}
            {checkResult.version && (
              <p>
                <strong>版本:</strong> {checkResult.version}
              </p>
            )}
          </div>
        )}
      </div>

      <div className="section">
        <h3>2. 下载 FFmpeg</h3>
        <button
          onClick={downloadFFmpeg}
          disabled={isDownloading || (checkResult?.available ?? false)}
        >
          {isDownloading ? "下载中..." : "下载 FFmpeg"}
        </button>

        {downloadProgress && (
          <div className="progress">
            <p>
              已下载: {(downloadProgress.downloaded / 1024 / 1024).toFixed(2)} MB
            </p>
            {downloadProgress.total && (
              <p>
                总大小: {(downloadProgress.total / 1024 / 1024).toFixed(2)} MB
              </p>
            )}
            {downloadProgress.percentage && (
              <>
                <div className="progress-bar">
                  <div
                    className="progress-fill"
                    style={{ width: `${downloadProgress.percentage}%` }}
                  ></div>
                </div>
                <p>{downloadProgress.percentage.toFixed(1)}%</p>
              </>
            )}
          </div>
        )}
      </div>

      <div className="section">
        <h3>3. 执行 FFmpeg 命令</h3>
        <button onClick={runFFmpegVersion} disabled={!checkResult?.available}>
          运行 ffmpeg -version
        </button>

        {executeResult && (
          <div className="result">
            <h4>输出:</h4>
            <pre>{executeResult}</pre>
          </div>
        )}

        {executeError && (
          <div className="result error">
            <h4>错误:</h4>
            <pre>{executeError}</pre>
          </div>
        )}
      </div>

      <div className="section">
        <h3>4. 删除 FFmpeg</h3>
        <button
          onClick={handleRemove}
          disabled={checkResult === null}
          className="delete-button"
        >
          删除 FFmpeg
        </button>
      </div>
    </div>
  );
}

export default FFmpegDemo;

