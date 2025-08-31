use crate::config as my_config;
use crate::window as my_window;
use anyhow::anyhow;
use once_cell::sync::Lazy;
use serde_json::json;
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

#[tauri::command(async)]
pub fn confirm_input(app: AppHandle, text: String, label: String) -> Result<(), String> {
    let app_handle_clone = app.clone();

    // 隐藏窗口
    my_window::hide_webview_window(app_handle_clone, &label)?;

    // 初始化模拟键盘输入插件
    // let mut enigo: Enigo = Enigo::new(&Settings::default()).map_err(|e| format!("Enigo初始化失败: {}", e))?;
    let mut enigo: std::sync::MutexGuard<'_, Enigo> = ENIGO
        .lock()
        .map_err(|e| format!("Enigo实例获取失败: {}", e))?;

    // 获取配置
    let config_label = label.replace("-", "_");
    let config_json = my_config::get_app_config_json(&app, &config_label).unwrap_or(json!({
        "typing_interval": "10",
        "enter_interval": "50",
        "pre_interval": "250"
    }));
    let typing_interval = extract_u64(&config_json, "typing_interval", 10);
    let enter_interval = extract_u64(&config_json, "enter_interval", 50);
    let pre_interval = extract_u64(&config_json, "pre_interval", 250);

    // println!(
    //     "------config_label, pre_interval, enter_interval, typing_interval, text-----：{}, {}, {}, {}, {}",
    //     config_label, pre_interval, enter_interval, typing_interval, &text
    // );

    // 打字输入必须要加这个 不然输入框没有字被输入进去（很奇怪
    sleep(Duration::from_millis(pre_interval));

    // 模拟回车唤起输入框：按下延时50ms左右释放，才能触发游戏里的唤起操作，很奇怪
    if enter_interval == 0 {
        println!("无延迟回车！");
        enigo
            .key(Key::Return, Click)
            .map_err(|e| format!("模拟回车点击失败: {}", e))?;
    } else {
        println!("延迟{}ms回车！", enter_interval);

        enigo
            .key(Key::Return, Press)
            .map_err(|e| format!("模拟回车按下失败: {}", e))?;
        sleep(Duration::from_millis(enter_interval));
        enigo
            .key(Key::Return, Release)
            .map_err(|e| format!("模拟回车抬起失败: {}", e))?;
        sleep(Duration::from_millis(enter_interval));
    }

    if typing_interval == 0 {
        // 直接输入文字
        enigo
            .text(&text)
            .map_err(|e| format!("模拟文字输入失败: {}", e))?;
    } else {
        // 有节奏的输入：每个字符间隔10ms，以应对部分玩家出现吞字的情况。
        for ch in text.chars() {
            enigo
                .text(&ch.to_string())
                .map_err(|e| format!("模拟文字输入失败: {}", e))?;
            sleep(Duration::from_millis(typing_interval));
        }
    }

    // 模拟回车发送文字
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
    Ok(())
}

fn extract_f64(val: &serde_json::Value, key: &str, default: f64) -> f64 {
    val.get(key)
        .and_then(|v| v.as_f64().or_else(|| v.as_str()?.parse::<f64>().ok()))
        .unwrap_or(default)
}

fn extract_u64(val: &serde_json::Value, key: &str, default: u64) -> u64 {
    val.get(key)
        .and_then(|v| v.as_u64().or_else(|| v.as_str()?.parse::<u64>().ok()))
        .unwrap_or(default)
}

// 注册全局快捷键
pub fn register_global_shortcut(app: &mut tauri::App) -> tauri::Result<()> {
    #[cfg(desktop)]
    {
        use tauri_plugin_global_shortcut::{
            Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState,
        };

        let keyboard_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::Space);
        let quickly_chat_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::Period);

        app.handle().plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |app_handle, shortcut, event| {
                    // println!("{:?}", shortcut);

                    // 按下快捷键执行相应的行为，只有抬起时才会执行，因为按键按住不放手可能会一直触发按下事件
                    if event.state == ShortcutState::Released {
                        println!("Released!");

                        if shortcut == &keyboard_shortcut {
                            println!("弹窗输入 Released!");

                            // 不存在则去创建窗口，否则切换窗口可视状态
                            if let Some(webview_window) = app_handle.get_webview_window("keyboard")
                            {
                                match webview_window.is_visible() {
                                    Ok(visible) => {
                                        // println!("弹窗输入-窗口可视状态: {}", !visible);
                                        if visible {
                                            let _ = webview_window.hide();
                                        } else {
                                            match webview_window.show() {
                                                Ok(_) => {
                                                    let _ = webview_window.set_focus();
                                                }
                                                Err(e) => {
                                                    println!("弹窗输入-窗口显示失败: {}", e);
                                                }
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        println!("弹窗输入-获取窗口可视状态失败: {}", e);
                                        let _ = webview_window.close(); // 存在则关闭
                                    }
                                }
                            } else {
                                let keyboard_json =
                                    my_config::get_app_config_json(app_handle, "keyboard".into())
                                        .unwrap_or(json!({}));
                                let width = extract_f64(&keyboard_json, "width", 340.0);
                                let height = extract_f64(&keyboard_json, "height", 40.0);
                                let x = extract_f64(&keyboard_json, "x", 1335.0);
                                let y = extract_f64(&keyboard_json, "y", 960.0);

                                let app_handle_clone = app_handle.clone();

                                // 创建异步任务：如果输入框不存在则去创建该窗口并聚焦
                                tauri::async_runtime::spawn(async move {
                                    let webview_window = tauri::WebviewWindowBuilder::new(
                                        &app_handle_clone,
                                        "keyboard",
                                        tauri::WebviewUrl::App("keyboard".into()),
                                    )
                                    .decorations(false)
                                    .transparent(true)
                                    .shadow(true)
                                    .always_on_top(true)
                                    .inner_size(width, height)
                                    .position(x, y)
                                    .build()
                                    .unwrap();

                                    let _ = webview_window.set_focus();
                                    println!("弹窗输入-窗口已创建");
                                });
                            }
                        }

                        if shortcut == &quickly_chat_shortcut {
                            println!("快捷聊天 Released!");

                            // 不存在则去创建窗口，否则切换窗口可视状态
                            if let Some(webview_window) =
                                app_handle.get_webview_window("quickly-chat")
                            {
                                match webview_window.is_visible() {
                                    Ok(visible) => {
                                        // println!("快捷聊天-窗口可视状态: {}", !visible);
                                        if visible {
                                            let _ = webview_window.hide();
                                        } else {
                                            match webview_window.show() {
                                                Ok(_) => {
                                                    let _ = webview_window.set_focus();
                                                }
                                                Err(e) => {
                                                    println!("快捷聊天-窗口显示失败: {}", e);
                                                }
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        println!("快捷聊天-获取窗口可视状态失败: {}", e);
                                        let _ = webview_window.close(); // 存在则关闭
                                    }
                                }
                            } else {
                                let quickly_chat_json = my_config::get_app_config_json(
                                    app_handle,
                                    "quickly_chat".into(),
                                )
                                .unwrap_or(json!({}));
                                let width = extract_f64(&quickly_chat_json, "width", 800.0);
                                let height: f64 = extract_f64(&quickly_chat_json, "height", 600.0);

                                let app_handle_clone = app_handle.clone();

                                // 创建异步任务：如果输入框不存在则去创建该窗口并聚焦
                                tauri::async_runtime::spawn(async move {
                                    let webview_window = tauri::WebviewWindowBuilder::new(
                                        &app_handle_clone,
                                        "quickly-chat",
                                        tauri::WebviewUrl::App("quickly-chat".into()),
                                    )
                                    .decorations(false) // 关闭系统边框和标题栏
                                    .transparent(true) // 窗口背景透明
                                    .shadow(true)
                                    .always_on_top(true)
                                    .inner_size(width, height)
                                    .center()
                                    .build()
                                    .unwrap();

                                    let _ = webview_window.set_focus();
                                    println!("快捷聊天-窗口已创建");
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

        let keyboard_json =
            my_config::get_app_config_json(app.handle(), "keyboard".into()).unwrap_or(json!({}));
        let keyboard_flag = keyboard_json
            .get("flag")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        if !app.global_shortcut().is_registered(keyboard_shortcut) && keyboard_flag {
            app.global_shortcut()
                .register(keyboard_shortcut)
                .map_err(|e| anyhow!("注册快捷键1失败: {}", e))?;
        } else {
            println!("{}该快捷键1已经注册过了/已关闭注册", keyboard_shortcut);
        }

        let quickly_chat_json = my_config::get_app_config_json(app.handle(), "quickly_chat".into())
            .unwrap_or(json!({}));
        let quickly_chat_flag = quickly_chat_json
            .get("flag")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        if !app.global_shortcut().is_registered(quickly_chat_shortcut) && quickly_chat_flag {
            app.global_shortcut()
                .register(quickly_chat_shortcut)
                .map_err(|e| anyhow!("注册快捷键2失败: {}", e))?;
        } else {
            println!("{}该快捷键2已经注册过了/已关闭注册", quickly_chat_shortcut);
        }
    }

    Ok(())
}
