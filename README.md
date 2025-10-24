# Tauri Plugin Use FFmpeg

一个为 Tauri v2 桌面应用提供使用 FFmpeg 功能的插件，支持自动下载和管理 FFmpeg 二进制文件。

## 功能特性

- ✅ 无需预装 FFmpeg
- ✅ 自动下载并解压 FFmpeg
- ✅ 支持桌面平台：macOS、Windows、Linux
- ✅ 实时下载进度监听
- ✅ FFmpeg 可用性检查（包含路径和版本信息）
- ✅ 执行任意 FFmpeg 命令
- ✅ 删除已下载的 FFmpeg
- ✅ 完整的 TypeScript 类型支持

## 安装

### Rust 依赖

在你的 `Cargo.toml` 中添加：

```toml
[dependencies]
tauri-plugin-use-ffmpeg = { path = "path/to/tauri-plugin-use-ffmpeg" }
```

### JavaScript 依赖

```bash
npm install tauri-plugin-use-ffmpeg-api
# 或
pnpm add tauri-plugin-use-ffmpeg-api
# 或
yarn add tauri-plugin-use-ffmpeg-api
```

## 使用方法

### 初始化插件（Rust）

在 `src-tauri/src/lib.rs` 中：

```rust
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_use_ffmpeg::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 权限配置

在 `src-tauri/capabilities/default.json` 中添加：

```json
{
  "permissions": [
    "core:default",
    "ffmpeg:default"
  ]
}
```

### 前端使用（TypeScript/JavaScript）

```typescript
import { check, download, execute, remove } from 'tauri-plugin-use-ffmpeg-api'

// 1. 检查 FFmpeg 是否可用
const checkResult = await check()
console.log('是否可用:', checkResult.available)
console.log('路径:', checkResult.path)
console.log('版本:', checkResult.version)

// 2. 如果不可用，下载 FFmpeg（带进度回调）
if (!checkResult.available) {
  await download(undefined, (progress) => {
    console.log(`已下载: ${progress.downloaded} 字节`)
    if (progress.total) {
      console.log(`总大小: ${progress.total} 字节`)
    }
    if (progress.percentage) {
      console.log(`进度: ${progress.percentage.toFixed(1)}%`)
    }
  })
}

// 3. 执行 FFmpeg 命令
const result = await execute(['-i', 'input.mp4', '-c:v', 'libx264', 'output.mp4'])
console.log('标准输出:', result.stdout)
console.log('标准错误:', result.stderr)
console.log('执行成功:', result.success)
console.log('退出码:', result.exitCode)

// 4. 删除 FFmpeg（如果需要）
const deleteResult = await remove()
console.log('删除成功:', deleteResult.success)
```

## API 文档

### TypeScript API

#### `check(): Promise<CheckResponse>`
检查 FFmpeg 是否已安装并可用。

返回：
- `available: boolean` - 是否可用
- `path?: string` - FFmpeg 可执行文件路径
- `version?: string` - FFmpeg 版本信息

#### `download(config?: DownloadConfig, onProgress?: (progress: DownloadProgress) => void): Promise<DownloadResponse>`
下载 FFmpeg 到本地。

参数：
- `config` (可选) - 自定义下载配置
- `onProgress` (可选) - 下载进度回调函数

返回：
- `success: boolean` - 是否成功
- `path?: string` - 下载后的文件路径
- `message?: string` - 消息

#### `execute(args: string[]): Promise<ExecuteResponse>`
执行 FFmpeg 命令。

参数：
- `args` - FFmpeg 命令参数数组（不包含 `ffmpeg` 本身）

返回：
- `success: boolean` - 是否成功
- `stdout: string` - 标准输出
- `stderr: string` - 标准错误输出
- `exitCode?: number` - 退出码

#### `remove(): Promise<DeleteResponse>`
删除已下载的 FFmpeg。

返回：
- `success: boolean` - 是否成功
- `message?: string` - 消息

### 默认下载配置

```typescript
DEFAULT_CONFIGS = {
  macos: {
    url: 'https://evermeet.cx/ffmpeg/ffmpeg-8.0.zip',
    executable_path: 'ffmpeg'
  },
  windows: {
    url: 'https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-n8.0-latest-win64-gpl-8.0.zip',
    executable_path: 'bin/ffmpeg.exe'
  }
}
```

### 自定义下载配置

```typescript
import { download } from 'tauri-plugin-use-ffmpeg-api'

// 使用自定义配置下载
await download(
  {
    url: 'https://your-custom-url.com/ffmpeg.zip',
    executable_path: 'path/in/zip/to/ffmpeg'
  },
  (progress) => {
    console.log(`下载进度: ${progress.percentage}%`)
  }
)
```

### 完整工作流示例

```typescript
import { check, download, execute, remove } from 'tauri-plugin-use-ffmpeg-api'

async function convertVideo() {
  // 检查 FFmpeg
  const checkResult = await check()
  
  // 如果未安装，先下载
  if (!checkResult.available) {
    console.log('FFmpeg 未安装，开始下载...')
    
    await download(undefined, (progress) => {
      if (progress.percentage) {
        console.log(`下载进度: ${progress.percentage.toFixed(1)}%`)
      }
    })
    
    console.log('FFmpeg 下载完成！')
  } else {
    console.log(`FFmpeg 已安装: ${checkResult.path}`)
    console.log(`版本: ${checkResult.version}`)
  }
  
  // 执行视频转换
  const result = await execute([
    '-i', 'input.mp4',
    '-c:v', 'libx264',
    '-crf', '23',
    '-preset', 'medium',
    'output.mp4'
  ])
  
  if (result.success) {
    console.log('视频转换成功！')
  } else {
    console.error('转换失败:', result.stderr)
  }
}
```

## 存储路径

FFmpeg 二进制文件会被下载到：
```
{app_data_dir}/bin/{platform}/ffmpeg[.exe]
```

例如：
- macOS: `~/Library/Application Support/com.your.app/bin/macos/ffmpeg`
- Windows: `C:\Users\{user}\AppData\Local\com.your.app\bin\windows\ffmpeg.exe`
- Linux: `~/.local/share/com.your.app/bin/linux/ffmpeg`

## 示例应用

查看 `examples/tauri-app` 目录获取完整的示例应用（React + TypeScript）。

运行示例：
```bash
cd examples/tauri-app
npm install
npm run tauri dev
```

## 注意事项

1. **平台支持**：此插件仅支持桌面平台（macOS、Windows、Linux），不支持移动端
2. **文件大小**：FFmpeg 压缩包较大（约 50-100MB），首次下载可能需要一些时间
3. **网络连接**：下载 FFmpeg 需要网络连接，建议在应用启动时检查并提示用户
4. **权限要求**：确保在 Tauri 配置中添加了 `ffmpeg:default` 权限

## 常见问题

### 如何处理下载失败？

```typescript
try {
  await download(undefined, (progress) => {
    console.log(`下载进度: ${progress.percentage}%`)
  })
} catch (error) {
  console.error('下载失败:', error)
  // 可以提示用户检查网络连接或重试
}
```

### 如何获取 FFmpeg 版本？

```typescript
const checkResult = await check()
if (checkResult.available && checkResult.version) {
  console.log('FFmpeg 版本:', checkResult.version)
}
```

### 如何使用自定义 FFmpeg 源？

```typescript
await download({
  url: 'https://your-cdn.com/ffmpeg.zip',
  executable_path: 'ffmpeg' // 或 'bin/ffmpeg.exe' (Windows)
})
```

## 许可证

MIT
