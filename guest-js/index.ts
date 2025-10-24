/**
 * Tauri FFmpeg 插件
 * 
 * 这个插件允许你在 Tauri 应用中使用 FFmpeg，而无需预装 FFmpeg。
 * 
 * 主要功能：
 * 1. 检查 FFmpeg 是否已安装 (check)
 * 2. 下载并安装 FFmpeg (download)
 * 3. 执行 FFmpeg 命令 (execute)
 * 4. 删除 FFmpeg (remove)
 * 
 * FFmpeg 会被下载到: app_data_dir/bin/{platform}/ffmpeg[.exe]
 * 
 * 默认下载地址：
 * - macOS: https://evermeet.cx/ffmpeg/ffmpeg-8.0.zip
 * - Windows: https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-n8.0-latest-win64-gpl-8.0.zip
 * 
 * 使用示例：
 * 
 * ```typescript
 * import { check, download, execute } from 'tauri-plugin-use-ffmpeg-api'
 * 
 * // 检查 FFmpeg 是否可用
 * const checkResult = await check()
 * if (!checkResult.available) {
 *   // 下载 FFmpeg，带进度回调
 *   await download(undefined, (progress) => {
 *     console.log(`下载进度: ${progress.percentage}%`)
 *   })
 * }
 * 
 * // 执行 FFmpeg 命令
 * const result = await execute(['-i', 'input.mp4', 'output.avi'])
 * console.log(result.stdout)
 * ```
 * 
 * @module tauri-plugin-use-ffmpeg-api
 */

import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'

/**
 * 下载配置接口
 */
export interface DownloadConfig {
  url: string
  executable_path: string
}

/**
 * 检查响应接口
 */
export interface CheckResponse {
  available: boolean
  path?: string
  version?: string
}

/**
 * 下载响应接口
 */
export interface DownloadResponse {
  success: boolean
  path?: string
  message?: string
}

/**
 * 执行响应接口
 */
export interface ExecuteResponse {
  success: boolean
  stdout: string
  stderr: string
  exitCode?: number
}

/**
 * 下载进度接口
 */
export interface DownloadProgress {
  downloaded: number
  total?: number
  percentage?: number
}

/**
 * 删除响应接口
 */
export interface DeleteResponse {
  success: boolean
  message?: string
}

/**
 * 默认下载配置
 */
export const DEFAULT_CONFIGS: Record<string, DownloadConfig> = {
  macos: {
    url: 'https://evermeet.cx/ffmpeg/ffmpeg-8.0.zip',
    executable_path: 'ffmpeg'
  },
  windows: {
    url: 'https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-n8.0-latest-win64-gpl-8.0.zip',
    executable_path: 'bin/ffmpeg.exe'
  }
}

/**
 * 检查 FFmpeg 是否已安装并可用
 * 
 * @returns 检查结果，包括是否可用、路径和版本信息
 * 
 * @example
 * ```typescript
 * import { check } from 'tauri-plugin-use-ffmpeg-api'
 * 
 * const result = await check()
 * console.log('是否可用:', result.available)
 * console.log('路径:', result.path)
 * console.log('版本:', result.version)
 * 
 * if (result.available) {
 *   console.log('FFmpeg 已安装，可以使用')
 * } else {
 *   console.log('FFmpeg 未安装，需要下载')
 * }
 * ```
 */
export async function check(): Promise<CheckResponse> {
  return await invoke<CheckResponse>('plugin:use-ffmpeg|check')
}

/**
 * 下载 FFmpeg
 * 
 * @param config 可选的下载配置，如果不提供则使用默认配置
 * @param onProgress 可选的进度回调函数
 * @returns 下载结果
 * 
 * @example
 * ```typescript
 * import { download } from 'tauri-plugin-use-ffmpeg-api'
 * 
 * // 使用默认配置下载
 * const result = await download()
 * console.log('下载成功:', result.success)
 * console.log('路径:', result.path)
 * 
 * // 带进度回调的下载
 * await download(undefined, (progress) => {
 *   console.log(`已下载: ${progress.downloaded} 字节`)
 *   if (progress.total) {
 *     console.log(`总大小: ${progress.total} 字节`)
 *   }
 *   if (progress.percentage) {
 *     console.log(`进度: ${progress.percentage.toFixed(1)}%`)
 *   }
 * })
 * 
 * // 使用自定义配置下载
 * await download({
 *   url: 'https://your-custom-url.com/ffmpeg.zip',
 *   executable_path: 'ffmpeg'
 * }, (progress) => {
 *   console.log(`下载进度: ${progress.percentage}%`)
 * })
 * ```
 */
export async function download(
  config?: DownloadConfig,
  onProgress?: (progress: DownloadProgress) => void
): Promise<DownloadResponse> {
  let unlisten: UnlistenFn | undefined

  if (onProgress) {
    unlisten = await listen<DownloadProgress>('use-ffmpeg://download-progress', (event) => {
      onProgress(event.payload)
    })
  }

  try {
    return await invoke<DownloadResponse>('plugin:use-ffmpeg|download', {
      payload: {
        config
      }
    })
  } finally {
    if (unlisten) {
      unlisten()
    }
  }
}

/**
 * 执行 FFmpeg 命令
 * 
 * @param args FFmpeg 命令参数数组（不包含 `ffmpeg` 本身）
 * @returns 执行结果
 * 
 * @example
 * ```typescript
 * import { execute } from 'tauri-plugin-use-ffmpeg-api'
 * 
 * // 获取 FFmpeg 版本
 * const versionResult = await execute(['-version'])
 * console.log(versionResult.stdout)
 * 
 * // 视频转换
 * const result = await execute([
 *   '-i', 'input.mp4',
 *   '-c:v', 'libx264',
 *   '-crf', '23',
 *   '-preset', 'medium',
 *   'output.mp4'
 * ])
 * 
 * if (result.success) {
 *   console.log('转换成功！')
 *   console.log('输出:', result.stdout)
 * } else {
 *   console.error('转换失败:', result.stderr)
 *   console.error('退出码:', result.exitCode)
 * }
 * 
 * // 提取音频
 * await execute([
 *   '-i', 'video.mp4',
 *   '-vn',
 *   '-acodec', 'libmp3lame',
 *   'audio.mp3'
 * ])
 * ```
 */
export async function execute(args: string[]): Promise<ExecuteResponse> {
  return await invoke<ExecuteResponse>('plugin:use-ffmpeg|execute', {
    payload: {
      args
    }
  })
}

/**
 * 删除已下载的 FFmpeg
 * 
 * @returns 删除结果
 * 
 * @example
 * ```typescript
 * import { remove, check } from 'tauri-plugin-use-ffmpeg-api'
 * 
 * // 删除 FFmpeg
 * const result = await remove()
 * console.log('删除成功:', result.success)
 * console.log('消息:', result.message)
 * 
 * // 删除后重新检查
 * const checkResult = await check()
 * console.log('FFmpeg 是否还存在:', checkResult.available)
 * 
 * // 带确认的删除
 * if (confirm('确定要删除 FFmpeg 吗？')) {
 *   const deleteResult = await remove()
 *   if (deleteResult.success) {
 *     console.log('FFmpeg 已删除')
 *   }
 * }
 * ```
 */
export async function remove(): Promise<DeleteResponse> {
  return await invoke<DeleteResponse>('plugin:use-ffmpeg|remove')
}
