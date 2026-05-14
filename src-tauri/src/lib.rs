use serde::{Deserialize, Serialize};

/// 音频缓存解析请求。
///
/// 这是为了兼容前端 `audio_resolve_cache` 调用而提供的最小结构。
/// 当前基础后端不会真正下载或缓存音频，只把前端传入的 URL 原样返回，
/// 这样可以保证应用能完成编译和基础运行。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioResolveCacheRequest {
    pub url: Option<String>,
    pub chapter_url: Option<String>,
    pub referer: Option<String>,
    pub user_agent: Option<String>,
}

/// 音频缓存解析响应。
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioResolveCacheResponse {
    pub local_path: String,
}

/// 返回当前运行平台。
///
/// 前端会通过 `get_platform` 判断运行环境；这里返回 Tauri/Rust 编译目标平台。
#[tauri::command]
fn get_platform() -> &'static str {
    #[cfg(target_os = "ios")]
    {
        return "ios";
    }

    #[cfg(target_os = "android")]
    {
        return "android";
    }

    #[cfg(target_os = "windows")]
    {
        return "windows";
    }

    #[cfg(target_os = "macos")]
    {
        return "macos";
    }

    #[cfg(target_os = "linux")]
    {
        return "linux";
    }

    "unknown"
}

/// 最小音频缓存解析命令。
///
/// 原项目可能计划在 Rust 侧实现音频代理下载 / 缓存。
/// 这里为了保证基础后端可打包，暂时不做网络请求，只返回 URL。
#[tauri::command]
fn audio_resolve_cache(request: AudioResolveCacheRequest) -> AudioResolveCacheResponse {
    let local_path = request
        .url
        .or(request.chapter_url)
        .or(request.referer)
        .unwrap_or_default();

    AudioResolveCacheResponse { local_path }
}

/// Tauri 应用入口。
///
/// 这里注册了前端当前能搜索到的最小命令：
/// - `get_platform`
/// - `audio_resolve_cache`
///
/// 同时启用前端 package.json 中已经依赖的常用插件，避免运行时插件 API 不可用。
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_platform, audio_resolve_cache])
        .run(tauri::generate_context!())
        .expect("error while running Legado Tauri application");
}
