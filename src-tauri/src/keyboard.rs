use crate::config as my_config;
use crate::window as my_window;
use anyhow::anyhow;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;
use tauri::{AppHandle, Manager};

use enigo::{
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Settings,
};
static ENIGO: Lazy<Mutex<Enigo>> =
    Lazy::new(|| Mutex::new(Enigo::new(&Settings::default()).unwrap()));

#[tauri::command]
pub fn confirm_input(app: AppHandle, text: String, label: String) -> Result<(), String> {
    // 隐藏窗口
    my_window::hide_webview_window(app, &label)?;
    sleep(Duration::from_millis(250));

    // let mut enigo: Enigo = Enigo::new(&Settings::default()).map_err(|e| format!("Enigo初始化失败: {}", e))?;
    let mut enigo = ENIGO
        .lock()
        .map_err(|e| format!("Enigo实例获取失败: {}", e))?;

    // 模拟回车唤起输入框：按下延时50ms左右释放，才能触发游戏里的唤起操作，很奇怪
    enigo
        .key(Key::Return, Press)
        .map_err(|e| format!("模拟回车按下失败: {}", e))?;
    sleep(Duration::from_millis(50));
    enigo
        .key(Key::Return, Release)
        .map_err(|e| format!("模拟回车抬起失败: {}", e))?;
    sleep(Duration::from_millis(50));

    // 直接输入前端传来的文字
    // enigo.text(&text).map_err(|e| format!("模拟文字输入失败: {}", e))?;
    // sleep(Duration::from_millis(250)).await;

    // 有节奏的输入：每个字符间隔10ms
    for ch in text.chars() {
        enigo
            .text(&ch.to_string())
            .map_err(|e| format!("模拟文字输入失败: {}", e))?;
        sleep(Duration::from_millis(10));
    }

    // 模拟回车发送文字
    sleep(Duration::from_millis(50));
    enigo
        .key(Key::Return, Click)
        .map_err(|e| format!("模拟回车点击失败: {}", e))?;

    Ok(())
}

// 模拟按下并释放 Enter 键
#[allow(dead_code)]
pub fn press_enter() -> Result<(), String> {
    let mut enigo = ENIGO
        .lock()
        .map_err(|e| format!("Enigo实例获取失败: {}", e))?;
    enigo
        .key(Key::Return, Press)
        .map_err(|e| format!("模拟回车按下失败: {}", e))?;
    sleep(Duration::from_millis(50));
    enigo
        .key(Key::Return, Release)
        .map_err(|e| format!("模拟回车抬起失败: {}", e))?;
    sleep(Duration::from_millis(50));
    Ok(())
}

// 注册全局快捷键
pub fn register_global_shortcut(app: &mut tauri::App) -> tauri::Result<()> {
    #[cfg(desktop)]
    {
        use tauri_plugin_global_shortcut::{
            Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState,
        };

        let ctrl_space_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::Space);
        let ctrl_slash_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::KeyP);

        app.handle().plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |app_handle, shortcut, event| {
                    println!("{:?}", shortcut);

                    // 按下快捷键执行相应的行为，只有抬起时才会执行，因为按键按住不放手可能会一直触发按下事件
                    if event.state == ShortcutState::Released {
                        println!("Released!");

                        if shortcut == &ctrl_space_shortcut {
                            println!("ctrl_space Released!");
                            if let Some(webview_window) = app_handle.get_webview_window("keyboard")
                            {
                                match webview_window.is_visible() {
                                    Ok(visible) => {
                                        println!("窗口可视旧状态: {}", visible);
                                        if visible {
                                            let _ = webview_window.hide();
                                        } else {
                                            match webview_window.show() {
                                                Ok(_) => {
                                                    let _ = webview_window.set_focus();
                                                }
                                                Err(e) => {
                                                    println!("窗口显示失败: {}", e);
                                                }
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        println!("获取窗口可视状态失败: {}", e);
                                        let _ = webview_window.close(); // 存在则关闭
                                    }
                                }
                            } else {
                                let app_handle_clone = app_handle.clone();

                                let my_config = my_config::get_app_config(app_handle)
                                    .unwrap_or_else(|err| {
                                        eprintln!("读取配置失败，使用默认值: {:?}", err);
                                        my_config::AppConfig::default()
                                    });

                                // 创建异步任务：如果输入框不存在则去创建该窗口并聚焦
                                tauri::async_runtime::spawn(async move {
                                    let params = my_window::WebviewWindowParams {
                                        label: "keyboard".into(),
                                        width: my_config
                                            .keyboard_width
                                            .parse::<f64>()
                                            .unwrap_or(340.0),
                                        height: my_config
                                            .keyboard_height
                                            .parse::<f64>()
                                            .unwrap_or(40.0),
                                        x: my_config.keyboard_x.parse::<f64>().unwrap_or(1335.0),
                                        y: my_config.keyboard_y.parse::<f64>().unwrap_or(960.0),
                                        decorations: false,
                                        transparent: true,
                                        shadow: false,
                                        always_on_top: true,
                                        url: "keyboard".into(),
                                        title: None,
                                    };

                                    my_window::create_webview_window(app_handle_clone, params)
                                        .await;
                                });
                            }
                        }

                        if shortcut == &ctrl_slash_shortcut {
                            println!("ctrl_slash Released!");
                            if let Some(webview_window) =
                                app_handle.get_webview_window("quickly-chat")
                            {
                                match webview_window.is_visible() {
                                    Ok(visible) => {
                                        println!("窗口可视旧状态: {}", visible);
                                        if visible {
                                            let _ = webview_window.hide();
                                        } else {
                                            match webview_window.show() {
                                                Ok(_) => {
                                                    let _ = webview_window.set_focus();
                                                }
                                                Err(e) => {
                                                    println!("窗口显示失败: {}", e);
                                                }
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        println!("获取窗口可视状态失败: {}", e);
                                        let _ = webview_window.close(); // 存在则关闭
                                    }
                                }
                            } else {
                                let app_handle_clone = app_handle.clone();

                                // 创建异步任务：如果输入框不存在则去创建该窗口并聚焦
                                tauri::async_runtime::spawn(async move {
                                    std::thread::spawn(move || {
                                        let webview_window = tauri::WebviewWindowBuilder::new(
                                            &app_handle_clone,
                                            "quickly-chat",
                                            tauri::WebviewUrl::App("quickly-chat".into()),
                                        )
                                        .decorations(false) // 关闭系统边框和标题栏
                                        .transparent(true) // 窗口背景透明
                                        .shadow(true)
                                        .always_on_top(true)
                                        .inner_size(800.0, 600.0)
                                        .center()
                                        .build()
                                        .unwrap();

                                        let _ = webview_window.set_focus();

                                        println!("窗口已创建");
                                    });
                                });
                            }
                        }
                    }
                })
                .build(),
        )?;

        app.global_shortcut()
            .unregister_all()
            .map_err(|e| anyhow!("取消所有快捷键注册失败: {}", e))?;

        if !app.global_shortcut().is_registered(ctrl_slash_shortcut) {
            app.global_shortcut()
                .register(ctrl_slash_shortcut)
                .map_err(|e| anyhow!("注册快捷键2失败: {}", e))?;
        } else {
            println!("{}该快捷键2已经注册过了", ctrl_slash_shortcut);
        }

        if !app.global_shortcut().is_registered(ctrl_space_shortcut) {
            app.global_shortcut()
                .register(ctrl_space_shortcut)
                .map_err(|e| anyhow!("注册快捷键1失败: {}", e))?;
        } else {
            println!("{}该快捷键1已经注册过了", ctrl_space_shortcut);
        }
    }

    Ok(())
}
