// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod fs;
mod db;
use crate::fs as my_fs;
use crate::db as my_db;


#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn my_custom_command() -> i32 {
    println!("I was invoked from JavaScript!");
    return 1;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            my_custom_command,
            my_fs::open_folder,
            my_fs::get_folder_first_image,
            my_fs::copy_files,
            my_fs::copy_and_rename_files,
            my_db::db_operate_test,
            my_db::get_mod_records,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
