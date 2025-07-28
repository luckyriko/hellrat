use enigo::{
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Settings,
};
use tauri::{AppHandle, Manager, Result};

use std::mem::size_of;
use windows::Win32::UI::Input::KeyboardAndMouse::*;

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
                    println!("快捷键被按下{:?}", shortcut);
                    // 按下快捷键执行相应的行为，只有抬起时才会执行，因为按键按住不放手可能会一直触发按下事件
                    if shortcut == &ctrl_space_shortcut {
                        match event.state() {
                            ShortcutState::Pressed => {
                                println!("ctrl_space_shortcut Pressed!");
                            }
                            ShortcutState::Released => {
                                println!("ctrl_space_shortcut Released!");

                                if let Some(webview_window) =
                                    app_handle.get_webview_window("keyboard")
                                {
                                    match webview_window.is_visible() {
                                        Ok(visible) => {
                                            println!("窗口是否可见: {}", visible);
                                            if visible {
                                                let _ = webview_window.hide();
                                            } else {
                                                match webview_window.show() {
                                                    Ok(_) => {
                                                        let _ = webview_window.set_focus();
                                                    }
                                                    Err(e) => {
                                                        println!("窗口聚焦失败: {}", e);
                                                    }
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            println!("获取窗口可见状态失败: {}", e);
                                            let _ = webview_window.close(); // 存在则关闭
                                        }
                                    }
                                } else {
                                    // 如果输入框不存在则去创建并聚焦
                                    let app_handle_clone = app_handle.clone();

                                    // 创建窗口的异步任务
                                    tauri::async_runtime::spawn(async move {
                                        let params = my_window::WebviewWindowParams {
                                            label: "keyboard".into(),
                                            width: 340.0,
                                            height: 40.0,
                                            x: 1335.0,
                                            y: 960.0,
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
                    }
                })
                .build(),
        )?;

        app.global_shortcut()
            .register(ctrl_space_shortcut)
            .map_err(|e| anyhow!("注册快捷键失败: {}", e))?;
    }

    Ok(())
}
