use anyhow::Result;
use once_cell::sync::OnceCell;
use serde_json::{Value, json};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::AppHandle;
use tauri::Manager;
use tauri_plugin_store::StoreExt;

// 全局变量，存储 identifier
static IDENTIFIER: OnceCell<Mutex<String>> = OnceCell::new();
// 供 main.rs 使用：初始化 identifier
pub fn set_identifier(id: String) {
    let _ = IDENTIFIER.set(Mutex::new(id));
}
// 供其他函数使用：获取 identifier
pub fn get_identifier() -> Option<String> {
    // let package_name = env!("CARGO_PKG_NAME"); //获取App名称
    IDENTIFIER.get().map(|m| m.lock().unwrap().clone())
}

// 手动解析配置文件
// pub fn get_app_config_json(app: &AppHandle) -> Result<Value> {
//     // 获取配置路径：例如 C:\Users\用户名\AppData\Roaming\MyApp\config.json
//     let config_path = app.path().resolve("config.json", BaseDirectory::AppData)?;
//     // println!("配置路径：{:?}", config_path);

//     // 读取文件内容
//     let content = fs::read_to_string(&config_path)
//         .with_context(|| format!("读取配置文件失败: {:?}", config_path))?;

//     // 解析为结构体
//     let config = serde_json::from_str(&content).with_context(|| format!("配置文件解析失败: {:?}", config_path))?;
//     // println!("{:?}", config);

//     Ok(config)
// }

pub fn get_app_config_json(app: &AppHandle, key: &str) -> Result<Value> {
    let store = app.store("config.json")?;
    let json_value = store.get(key).unwrap_or(json!({}));
    Ok(json_value)
}

pub fn get_app_config_path(app: &AppHandle) -> Option<PathBuf> {
    app.path().app_config_dir().ok()
}

pub fn get_app_data_path(app: &AppHandle) -> Option<PathBuf> {
    app.path().app_data_dir().ok()
}

pub fn initialization_config_json(app: &AppHandle) -> Result<()> {
    let store = app.store("config.json")?;

    let game_mod_value = store.get("game_mod");
    if game_mod_value.is_none() {
        let app_config_path = get_app_config_path(app).unwrap();

        // 如果mods目录不存在则创建
        let mut app_data_path = get_app_data_path(app).unwrap();
        let mut app_data_path_clone = app_data_path.clone();

        app_data_path.push("mods");
        let mod_store_path = app_data_path.to_string_lossy().to_string();
        if !app_data_path.exists() {
            fs::create_dir_all(&mod_store_path)?;
        }

        app_data_path_clone.push("temp");
        let mods_temp_cache_path = app_data_path_clone.to_string_lossy().to_string();
        if !app_data_path_clone.exists() {
            fs::create_dir_all(&mods_temp_cache_path)?;
        }

        store.set(
            "game_mod",
            json!({
                "app_config_path": app_config_path.to_string_lossy().to_string(),
                "game_data_path": "",
                "mods_store_path": mod_store_path,
                "mods_temp_cache_path": mods_temp_cache_path,
                "mods_install_priority": "asc"
            }),
        );
    }

    let keyboard_value = store.get("keyboard");
    if keyboard_value.is_none() {
        store.set(
            "keyboard",
            json!({
                "flag": true,
                "shortcut": "ctrl + space",
                "typing_interval": "10",
                "width": "340.0",
                "height": "40.0",
                "x": "1335.0",
                "y": "960.0",
            }),
        );
    }

    let quickly_chat_value = store.get("quickly_chat");
    if quickly_chat_value.is_none() {
        store.set(
            "quickly_chat",
            json!({
                "flag": true,
                "shortcut": "ctrl + .",
                "typing_interval": "10",
                "width": "800.0",
                "height": "600.0",
                "chat":{
                    "chat1": "你好！",
                    "chat2": "再见！",
                    "chat3": "Ciallo～(∠・ω< )⌒☆",
                    "chat4": "",
                    "chat5": "",
                    "chat6": "",
                    "chat7": "",
                    "chat8": "",
                    "chat9": ""
                }

            }),
        );
    }

    Ok(())
}
