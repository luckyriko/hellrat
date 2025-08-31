use tauri::{AppHandle, Manager};

// 隐藏窗口
#[tauri::command()]
pub fn hide_webview_window(app: AppHandle, label: &str) -> Result<(), String> {
    if let Some(webview_window) = app.get_webview_window(label) {
        webview_window
            .hide()
            .map_err(|e| format!("窗口隐藏失败: {}", e))?;
        println!("窗口 {} 已隐藏", label);
    } else {
        println!("未找到窗口：{}", label);
    }
    Ok(())
}

// 关闭窗口
#[tauri::command()]
pub fn close_webview_window(app: AppHandle, label: &str) -> Result<(), String> {
    if let Some(webview_window) = app.get_webview_window(label) {
        webview_window
            .close()
            .map_err(|e| format!("窗口关闭失败: {}", e))?;
        println!("窗口 {} 已关闭", label);
    } else {
        println!("未找到窗口：{}", label);
    }

    Ok(())
}

// 聚焦已存在的窗口
#[tauri::command()]
pub fn focus_if_webview_window_exists(app: AppHandle, label: &str) -> bool {
    if let Some(webview_window) = app.get_webview_window(label) {
        let _ = webview_window.set_focus();
        true
    } else {
        false
    }
}
