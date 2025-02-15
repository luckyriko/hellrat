use rusqlite::Connection;
extern crate dirs;
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct GameMod {
    pub id: Option<i64>,
    pub name: String,
    pub mod_type: String,
    pub activate: bool,
    pub author: String,
    pub link: String,
    pub desc: String,
    pub preview: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameModData {
    pub info: GameMod,
    // pub no_array: Vec<String>,
    pub files: Vec<String>,
}

#[tauri::command]
pub fn db_operate_test() {

    get_mod_records();
}

/// 记录mod信息
#[tauri::command]
pub fn get_mod_records() -> Result<Vec<GameMod>, String> {
    create_db_table_if_not_exists().map_err(|e| format!("数据库初始化失败: {}", e))?;

    let conn = open_data_db().map_err(|e| format!("数据库连接失败: {}", e))?;

    let mut stmt = conn.prepare("SELECT * FROM mods_records ORDER BY id DESC").map_err(|e| format!("数据库创建查询失败: {}", e))?;
    let records_iter = stmt.query_map([], |row| {
        Ok(GameMod {
            id: row.get(0).ok(),
            name: row.get(1)?,
            mod_type: row.get(2)?,
            activate: row.get(3)?,
            author: row.get(4)?,
            link: row.get(5)?,
            desc: row.get(6)?,
            preview: row.get(7)?,
        })
    }).map_err(|e| format!("数据查询失败: {}", e))?;

    let records = records_iter.filter_map(Result::ok).collect();

    Ok(records)
}

/// 记录mod信息
fn add_mod_records(conn: &Connection, game_mod: &GameModData) -> Result<i64> {
    conn.execute(
        "INSERT INTO mods_records (name, mod_type, activate, author, link, desc, preview) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        (&game_mod.info.name, &game_mod.info.mod_type, u8::from(*(&game_mod.info.activate)) , &game_mod.info.author, &game_mod.info.link, &game_mod.info.desc, &game_mod.info.preview),
    )?;
    let last_record_id = conn.last_insert_rowid();
    // println!("last_record_id: {last_record_id}");

    Ok(last_record_id)
}

/// 记录mod安装的名称和模型序列号
// fn add_mod_install(conn: &Connection, game_mod: &GameModData, last_record_id: i64) -> Result<i64> {
//     conn.execute(
//         "INSERT INTO mods_install (name, mod_type, record_id) VALUES (?1, ?2, ?3)",
//         (&game_mod.info.name, &game_mod.info.mod_type, last_record_id),
//     )?;
//     let last_install_id = conn.last_insert_rowid();
//     // println!("last_install_id: {last_install_id}");

//     Ok(last_install_id)
// }

/// 记录mod所安装的文件
fn add_mod_install_files(conn: &Connection, game_mod: &GameModData, record_id: i64) -> Result<()> {
    for file in &game_mod.files {
        println!("add_mod_install_files：{}", file);
        conn.execute(
            "INSERT INTO mods_install_files (record_id, file ) VALUES (?1, ?2)",
            (record_id, file),
        )?;
    }

    Ok(())
}

pub fn add_mod(game_mod: GameModData) -> Result<()> {
    create_db_table_if_not_exists().context("create data.db failed")?;

    let mut conn = open_data_db()?;
    let tx = conn.transaction()?;

    // 存档记录
    let record_id = add_mod_records(&tx, &game_mod)?; // tx causes rollback if this fails

    // 安装记录
    if game_mod.info.activate {
        // let install_id = add_mod_install(&tx, &game_mod, record_id)?; // tx causes rollback if this fails
        add_mod_install_files(&tx, &game_mod, record_id)?; // tx causes rollback if this fails
    }

    tx.commit()?;
    Ok(())
}

pub fn create_db_table_if_not_exists() -> Result<()> {
    let conn = open_data_db()?;

    // mods存档，包含：名称、类型、安装状态、作者、链接、详情、预览图
    conn.execute(
        "CREATE TABLE IF NOT EXISTS mods_records (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            name        TEXT NOT NULL,
            mod_type    TEXT NOT NULL,
            activate    INTEGER NOT NULL,
            author      TEXT,
            link        TEXT,
            desc        TEXT,
            preview     TEXT
        )",
        (),
    )?;

    // 已安装mods记录表，record_id用于查找展示mod信息
    // conn.execute(
    //     "CREATE TABLE IF NOT EXISTS mods_install (
    //         id          INTEGER PRIMARY KEY AUTOINCREMENT,
    //         name        TEXT NOT NULL,
    //         mod_type    TEXT NOT NULL,
    //         no_array    TEXT,
    //         record_id   INTEGER

    // )",
    //     (),
    // )?;

    // 已安装mods文件记录表，install_id用于表明哪些文件属于哪条记录
    conn.execute(
        "CREATE TABLE IF NOT EXISTS mods_install_files (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            record_id   INTEGER NOT NULL,
            file        TEXT NOT NULL

    )",
        (),
    )?;

    Ok(())
}

pub fn open_data_db() -> Result<Connection> {
    // let platform = tauri_plugin_os::platform();
    // println!("Platform: {}", platform);

    if let Some(mut db_path) = dirs::config_dir() {
        db_path.push("com.luckyriko.helldivers2");
        db_path.push("db");
        let path = db_path.to_string_lossy().to_string();

        if !db_path.exists() {
            // 使用 create_dir_all 创建所有需要的嵌套目录
            fs::create_dir_all(path)?;
            println!("Directory created");
        } else {
            println!("Directory already exists");
        }

        // sqlite3的db文件路径=db_path
        db_path.push("data.db");
        println!("{:?}", db_path);
        let path = db_path.to_string_lossy().to_string();
        let conn = Connection::open(path).context("Failed to open data.db")?;

        Ok(conn)
    } else {
        Err(anyhow!("Failed to get appConfigDir"))
    }
}

// #[tauri::command]
// pub fn test(path: String) -> Result<()>  {
//     let conn = Connection::open_in_memory()?;

//     conn.execute(
//         "CREATE TABLE person (
//             id    INTEGER PRIMARY KEY,
//             name  TEXT NOT NULL,
//             data  BLOB
//         )",
//         (), // empty list of parameters.
//     )?;
//     let me = Person {
//         id: 0,
//         name: "Steven".to_string(),
//         data: None,
//     };
//     conn.execute(
//         "INSERT INTO person (name, data) VALUES (?1, ?2)",
//         (&me.name, &me.data),
//     )?;

//     let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
//     let person_iter = stmt.query_map([], |row| {
//         Ok(Person {
//             id: row.get(0)?,
//             name: row.get(1)?,
//             data: row.get(2)?,
//         })
//     })?;

//     for person in person_iter {
//         println!("Found person {:?}", person.unwrap());
//     }
//     Ok(())
// }
