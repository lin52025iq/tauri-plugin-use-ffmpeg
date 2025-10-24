use tauri::{command, AppHandle, Runtime};

use crate::models::*;
use crate::FfmpegExt;
use crate::Result;

#[command]
pub(crate) async fn check<R: Runtime>(app: AppHandle<R>) -> Result<CheckResponse> {
    app.ffmpeg().check()
}

#[command]
pub(crate) async fn download<R: Runtime>(
    app: AppHandle<R>,
    payload: DownloadRequest,
) -> Result<DownloadResponse> {
    app.ffmpeg().download(payload).await
}

#[command]
pub(crate) async fn execute<R: Runtime>(
    app: AppHandle<R>,
    payload: ExecuteRequest,
) -> Result<ExecuteResponse> {
    app.ffmpeg().execute(payload)
}

#[command]
pub(crate) async fn remove<R: Runtime>(app: AppHandle<R>) -> Result<DeleteResponse> {
    app.ffmpeg().remove()
}
