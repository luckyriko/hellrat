// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod db;
mod fs;
mod keyboard;
mod window;
mod config;

use crate::db as my_db;
use crate::fs as my_fs;
use crate::window as my_window;
use crate::keyboard as my_keyboard;

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

fn init_database() -> tauri::Result<()> {
    my_db::create_db_table_if_not_exists()?;
    println!("init_database数据库初始化！");
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
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
            my_fs::get_mod_info,
            my_fs::up_mod_info,
            my_fs::down_copy_and_rename_files,
            my_fs::remove_dir_all,
            my_fs::uninstall_mods_all,
            my_fs::deploy_mods,
            my_fs::unzip_one_file,
            my_db::db_operate_test,
            my_db::get_mod_records,
            my_db::get_mod_install_files,
            my_db::check_mod_name,
            my_db::get_statistics,
            my_window::close_webview_window,
            my_keyboard::confirm_input,
        ])
        .setup(|app| {
            println!("Tauri 初始化逻辑执行！");

            // let app_handle = app.handle().clone();
            // config::load_config(app_handle)?;
            // 数据库初始化
            init_database()?;

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
                        // 你想执行的逻辑，比如释放资源、发事件等
                        println!("{} 窗口已关闭", label);
                        // enigo.key(Key::Return, Click).unwrap();
                        // enigo.text("\n").unwrap();

                        // 获取全局 AppHandle
                        // let app_handle = window.app_handle();
                        // // 访问共享状态
                        // if let Some(state) = app_handle.try_state::<Arc<Mutex<KeyboardInput>>>() {
                        //     let text = state.lock().unwrap().text.clone();
                        //     println!("当前共享数据为: {}", text);

                        //     thread::sleep(Duration::from_millis(200));
                        //     let mut enigo = Enigo::new(&Settings::default()).unwrap();
                        //     enigo.text(&text).unwrap();
                        //     thread::sleep(Duration::from_millis(200));
                        //     enigo.key(Key::Return, Click).unwrap();
                        // }
                    }
                    _ => {}
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
