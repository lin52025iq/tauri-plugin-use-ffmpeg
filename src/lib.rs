//! # Tauri Plugin Use FFmpeg
//!
//! 一个为 Tauri v2 桌面应用提供使用 FFmpeg 功能的插件，支持自动下载和管理 FFmpeg 二进制文件。
//!
//! ## 功能特性
//!
//! - ✅ 无需预装 FFmpeg
//! - ✅ 自动下载并解压 FFmpeg
//! - ✅ 支持桌面平台：macOS、Windows、Linux
//! - ✅ 实时下载进度监听
//! - ✅ FFmpeg 可用性检查（包含路径和版本信息）
//! - ✅ 执行任意 FFmpeg 命令
//! - ✅ 删除已下载的 FFmpeg
//! - ✅ 完整的 TypeScript 类型支持
//!
//! ## 使用方法
//!
//! ### 初始化插件
//!
//! ```rust,ignore
//! use tauri_plugin_use_ffmpeg::init;
//!
//! fn main() {
//!     tauri::Builder::default()
//!         .plugin(init())
//!         .run(tauri::generate_context!())
//!         .expect("error while running tauri application");
//! }
//! ```
//!
//! ### 权限配置
//!
//! 在 `src-tauri/capabilities/default.json` 中添加：
//!
//! ```json
//! {
//!   "permissions": [
//!     "core:default",
//!     "use-ffmpeg:default"
//!   ]
//! }
//! ```

use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;

mod commands;
mod desktop;
mod error;
mod models;

pub use error::{Error, Result};

use desktop::Ffmpeg;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the ffmpeg APIs.
pub trait FfmpegExt<R: Runtime> {
    fn ffmpeg(&self) -> &Ffmpeg<R>;
}

impl<R: Runtime, T: Manager<R>> crate::FfmpegExt<R> for T {
    fn ffmpeg(&self) -> &Ffmpeg<R> {
        self.state::<Ffmpeg<R>>().inner()
    }
}

/// Initializes the plugin.
///
/// # Example
///
/// ```rust,ignore
/// use tauri_plugin_use_ffmpeg::init;
///
/// fn main() {
///     tauri::Builder::default()
///         .plugin(init())
///         .run(tauri::generate_context!())
///         .expect("error while running tauri application");
/// }
/// ```
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("use-ffmpeg")
        .invoke_handler(tauri::generate_handler![
            commands::check,
            commands::download,
            commands::execute,
            commands::remove
        ])
        .setup(|app, api| {
            let ffmpeg = desktop::init(app, api)?;
            app.manage(ffmpeg);
            Ok(())
        })
        .build()
}
