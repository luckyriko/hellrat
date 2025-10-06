// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod config;
mod db;
mod fs;
mod keyboard;
mod mods;
mod window;

use crate::config as my_config;
use crate::db as my_db;
use crate::fs as my_fs;
use crate::keyboard as my_keyboard;
use crate::mods as my_mods;
use crate::window as my_window;

use tauri::{Manager, Window, WindowEvent};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};
use tauri_plugin_global_shortcut::GlobalShortcutExt;

fn on_window_event(window: &Window, event: &WindowEvent) {
    // https://v2.tauri.app/develop/state-management/#access-state-with-the-manager-trait
    let app_handle = window.app_handle();

    let label = window.label();
    if label == "main" {
        match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                //阻止默认关闭
                api.prevent_close();
                let answer = window
                    .dialog()
                    .message("是否关闭应用？")
                    .title("提示")
                    .buttons(MessageDialogButtons::OkCancelCustom(
                        "确认".to_string(),
                        "取消".to_string(),
                    ))
                    .blocking_show();
                // println!("answer {}", answer);
                if answer {
                    let shortcut_unregister_all_res = app_handle.global_shortcut().unregister_all();
                    if let Err(e) = shortcut_unregister_all_res {
                        println!("注销所有快捷键失败: {:#?}", e);
                    }

                    for webview_window in app_handle.webview_windows().values() {
                        println!("循环查看所有窗口: {}", webview_window.label());
                        let res = webview_window.destroy();
                        if let Err(e) = res {
                            println!("关闭窗口失败: {:#?}", e);
                        }
                    }

                    app_handle.exit(0);
                    // app_handle.restart();
                }
            }
            _ => {}
        }
    }
}

// fn get_monitor_info(window: Window) -> Result<String, String> {
//     match window.current_monitor() {
//         Ok(Some(monitor)) => {
//             let size = monitor.size();
//             let position = monitor.position();
//             Ok(format!(
//                 "Monitor size: {}x{}, position: x={}, y={}",
//                 size.width, size.height, position.x, position.y
//             ))
//         }
//         Ok(None) => Err("No monitor found".into()),
//         Err(e) => Err(format!("Error getting monitor: {}", e)),
//     }
// }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }))
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            my_fs::open_folder,
            my_fs::get_seven_zip_path,
            my_db::db_operate_test,
            my_db::get_mods_records_with_env_add_flag,
            my_db::get_environment_mods_records,
            my_db::update_environment_activate,
            my_db::update_environment_name,
            my_db::get_environment_list,
            my_db::update_environment_mod_activate,
            my_db::update_environment_mod_options,
            my_window::close_webview_window,
            my_window::focus_if_webview_window_exists,
            my_keyboard::confirm_input,
            my_mods::save_mods,
            my_mods::get_env_mod_info,
            my_mods::get_env_mod_info_with_install_files,
            my_mods::install_mods,
            my_mods::up_mod_info,
            my_mods::uninstall_mods,
            my_mods::delete_one_mod,
            my_mods::update_environment_mod_sort,
            my_mods::add_environment,
            my_mods::copy_environment,
            my_mods::delete_environment,
            my_mods::add_mods_to_environment
        ])
        .setup(|app| {
            println!("Tauri 初始化逻辑执行！");

            // 应用唯一标识获取并保存
            let app_identifier = app.config().identifier.clone();
            println!("app_identifier值是 {}", app_identifier);
            my_config::set_identifier(app_identifier);

            // 配置文件初始化
            let app_handle = app.handle();
            my_config::initialization_config_json(app_handle)?;

            // 数据库初始化
            my_db::init_database()?;

            // 注册快捷键
            let register_status = my_keyboard::register_global_shortcut(app);
            match register_status {
                Ok(_) => {
                    println!("注册快捷键成功");
                }
                Err(e) => {
                    println!("注册快捷键失败: {}", e);
                }
            }

            Ok(())
        })
        .on_window_event(on_window_event)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
