// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod db;
mod fs;
use crate::db as my_db;
use crate::fs as my_fs;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn my_custom_command() -> i32 {
    println!("I was invoked from JavaScript!");
    return 1;
}

fn init_database(_app: &tauri::AppHandle) -> tauri::Result<()> {
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
            my_db::db_operate_test,
            my_db::get_mod_records,
            my_db::get_mod_install_files,
            my_db::check_mod_name,
            my_db::get_statistics,
        ])
        .setup(|app| {
            println!("Tauri 初始化逻辑执行！");

            // 你可以在这里执行数据库初始化、全局变量设置等
            let handle = app.handle();
            init_database(&handle)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
