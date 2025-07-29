use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, Result};

#[derive(Serialize, Deserialize, Debug)]
pub struct WebviewWindowParams {
    pub label: String,
    pub width: f64,
    pub height: f64,
    pub x: f64,
    pub y: f64,
    pub decorations: bool,
    pub transparent: bool,
    pub shadow: bool,
    pub always_on_top: bool,
    pub url: String,
    pub title: Option<String>,
}

impl Default for WebviewWindowParams {
    fn default() -> Self {
        Self {
            label: "default".into(),
            width: 800.0,
            height: 600.0,
            x: 0.0,
            y: 0.0,
            decorations: false,
            transparent: false,
            shadow: false,
            always_on_top: false,
            url: "/".into(),
            title: None,
        }
    }
}

// 创建窗口
#[tauri::command(async)]
pub async fn create_webview_window(app_handle: AppHandle, params: WebviewWindowParams) {
    // println!("{:#?}", params);

    std::thread::spawn(move || {
        let webview_window = tauri::WebviewWindowBuilder::new(
            &app_handle,
            params.label,
            tauri::WebviewUrl::App(params.url.into()),
        )
        .decorations(params.decorations)
        .transparent(params.transparent)
        .shadow(params.shadow)
        .always_on_top(params.always_on_top)
        .inner_size(params.width, params.height)
        .position(params.x, params.y)
        .build()
        .unwrap();

        let _ = webview_window.set_focus();

        println!("窗口已创建");
    });
}

// 隐藏窗口
#[tauri::command()]
pub fn hide_webview_window(app: AppHandle, label: &str) -> Result<()> {
    if let Some(webview_window) = app.get_webview_window(label) {
        webview_window.hide()?;
        println!("窗口 {} 已隐藏", label);
    } else {
        println!("未找到窗口：{}", label);
    }
    Ok(())
}

// 关闭窗口
#[allow(dead_code)]
#[tauri::command()]
pub fn close_webview_window(app: AppHandle, label: &str) -> Result<()> {
    if let Some(webview_window) = app.get_webview_window(label) {
        webview_window.close()?;
        println!("窗口 {} 已关闭", label);
    } else {
        println!("未找到窗口：{}", label);
    }

    Ok(())
}
