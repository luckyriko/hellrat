use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri::{Manager, path::BaseDirectory};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub gameDataDir: String,
    pub modsDir: String,
    pub keyboard_width: String,
    pub keyboard_height: String,
    pub keyboard_x: String,
    pub keyboard_y: String,

}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            gameDataDir: "".into(),
            modsDir: "".into(),
            keyboard_width: "340.0".into(),
            keyboard_height: "40.0".into(),
            keyboard_x: "1335.0".into(),
            keyboard_y: "960.0".into(),
        }
    }
}
// AppConfig
pub fn get_app_config(app: &AppHandle) -> Result<AppConfig> {
    // 获取配置路径：例如 C:\Users\用户名\AppData\Roaming\MyApp\config.json
    let config_path = app
        .path()
        .resolve("config.json", BaseDirectory::AppData)?;

    // println!("配置路径：{:?}", config_path);

    // 读取文件内容
    let content = fs::read_to_string(&config_path).with_context(|| format!("读取配置文件失败: {:?}", config_path))?;

    // 解析为结构体
    let config: AppConfig = serde_json::from_str(&content).context("配置文件格式错误")?;
    // println!("{:?}", config);

    Ok(config)
}
