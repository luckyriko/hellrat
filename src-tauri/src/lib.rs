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

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn my_custom_command() -> i32 {
    println!("I was invoked from JavaScript!");
    return 1;
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
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            my_custom_command,
            my_fs::open_folder,
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
            let app_handle_clone = app.handle();
            my_config::initialization_config_json(app_handle_clone)?;

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
        .on_window_event(|window, event| {
            let label = window.label();
            if label == "keyboard" {
                match event {
                    tauri::WindowEvent::Destroyed { .. } => {
                        println!("{} 窗口已关闭", label);
                    }
                    _ => {}
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
