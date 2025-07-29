use enigo::{
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Settings,
};
use tauri::{AppHandle, Manager, Result};

use std::mem::size_of;
use windows::Win32::UI::Input::KeyboardAndMouse::*;

use crate::config as my_config;
use crate::window as my_window;

use anyhow::anyhow;

use tokio::time::{Duration, sleep};

#[tauri::command]
pub async fn confirm_input(app: AppHandle, text: String) -> Result<()> {
    // 隐藏窗口
    my_window::hide_webview_window(app, "keyboard")?;

    sleep(Duration::from_millis(200)).await;

    let mut enigo: Enigo = Enigo::new(&Settings::default()).unwrap();

    // 唤起输入框：按下延时50ms左右释放，才能触发游戏里的唤起操作，很奇怪
    enigo.key(Key::Return, Press).unwrap();
    sleep(Duration::from_millis(50)).await;
    enigo.key(Key::Return, Release).unwrap();

    // 输入前端传来的文字
    enigo.text(&text).unwrap();

    // 自动回车发送文字
    enigo.key(Key::Return, Click).unwrap();

    Ok(())
}

// 模拟按下并释放 Enter 键
#[allow(dead_code)]
pub fn press_enter() -> windows::core::Result<()> {
    unsafe {
        let inputs = [
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VIRTUAL_KEY(13),
                        wScan: 0,
                        dwFlags: KEYBD_EVENT_FLAGS(0), // 按下
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            },
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VIRTUAL_KEY(13),
                        wScan: 0,
                        dwFlags: KEYEVENTF_KEYUP, // 松开
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            },
        ];

        let sent = SendInput(&inputs, size_of::<INPUT>() as i32);
        if sent == inputs.len() as u32 {
            Ok(())
        } else {
            Err(windows::core::Error::from_win32())
        }
    }
}

// 注册全局快捷键
pub fn register_global_shortcut(app: &mut tauri::App) -> Result<()> {
    #[cfg(desktop)]
    {
        use tauri_plugin_global_shortcut::{
            Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState,
        };

        let ctrl_space_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::Space);

        app.handle().plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |app_handle, shortcut, event| {
                    // 按下快捷键执行相应的行为，只有抬起时才会执行，因为按键按住不放手可能会一直触发按下事件
                    if event.state == ShortcutState::Released {
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
                                        width: my_config.keyboard_width.parse::<f64>().unwrap_or(340.0),
                                        height: my_config.keyboard_height.parse::<f64>().unwrap_or(40.0),
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
                    }
                })
                .build(),
        )?;

        if !app.global_shortcut().is_registered(ctrl_space_shortcut) {
            app.global_shortcut()
                .register(ctrl_space_shortcut)
                .map_err(|e| anyhow!("注册快捷键失败: {}", e))?;
        } else {
            println!("{}该快捷键已经注册过了", ctrl_space_shortcut);
        }
    }

    Ok(())
}
