use rusqlite::{params, params_from_iter, Connection, ToSql};
extern crate dirs;
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct GameMod {
    pub id: Option<i64>,
    pub name: String,
    pub memo: String,
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
pub fn db_operate_test() {}

pub fn check_mod_activate(record_id: i64) -> Result<(), String> {
    let conn = open_data_db().map_err(|e| format!("数据库连接失败: {}", e))?;

    let mut stmt = conn
        .prepare("SELECT activate FROM mods_records WHERE id = ?1")
        .map_err(|e| format!("删除时查询安装状态失败: {}", e))?;
    let activate: i64 = stmt
        .query_row(params![record_id], |row| row.get(0))
        .map_err(|e| format!("删除时查询安装状态失败: {}", e))?;

    println!("activate:{}", activate);
    if activate == 1 {
        return Err("该mod正在使用，请先卸载后再删除".to_string());
    }
    Ok(())
}

#[tauri::command]
pub fn check_mod_name(name: &str) -> Result<bool, String> {
    let conn = open_data_db().map_err(|e| format!("数据库连接失败: {}", e))?;

    let mut stmt = conn
        .prepare("SELECT COUNT(*) as count FROM mods_records WHERE name = ?1")
        .map_err(|e| format!("查询是否有重复名称失败: {}", e))?;
    let count: i32 = stmt
        .query_row(params![name], |row| row.get(0))
        .map_err(|e| format!("查询是否有重复名称失败: {}", e))?;

    // println!("count:{}", count);
    if count == 0 {
        Ok(true)
    } else {
        Ok(false)
    }
}

#[derive(Serialize)]
pub struct Statistic {
    records_activate_count: i32,
    records_total_count: i32,
    model_activate_count: i32,
    model_total_count: i32,
    voice_activate_count: i32,
    voice_total_count: i32,
}

#[tauri::command]
pub fn get_statistics() -> Result<Statistic, String> {
    let conn = open_data_db().map_err(|e| format!("数据库连接失败: {}", e))?;

    let mut stmt = conn
        .prepare(
            "SELECT 
            (SELECT COUNT(*) FROM mods_records WHERE activate=1),
            (SELECT COUNT(*) FROM mods_records),
            (SELECT COUNT(*) FROM mods_records WHERE mod_type = 'model' AND activate=1),
            (SELECT COUNT(*) FROM mods_records WHERE mod_type = 'model'),
            (SELECT COUNT(*) FROM mods_records WHERE mod_type = 'voice' AND activate=1 ),
            (SELECT COUNT(*) FROM mods_records WHERE mod_type = 'voice')
            ",
        )
        .map_err(|e| format!("预编译统计SQL失败: {}", e))?;

    let data = stmt
        .query_row([], |row| {
            Ok(Statistic {
                records_activate_count: row.get(0)?,
                records_total_count: row.get(1)?,
                model_activate_count: row.get(2)?,
                model_total_count: row.get(3)?,
                voice_activate_count: row.get(4)?,
                voice_total_count: row.get(5)?,
            })
        })
        .map_err(|e| format!("统计失败: {}", e))?;

    Ok(data)
}

/// 查询mod安装的文件名
#[tauri::command]
pub fn get_mod_install_files(record_id: i64) -> Result<Vec<String>, String> {
    let conn = open_data_db().map_err(|e| format!("数据库连接失败: {}", e))?;

    let mut stmt = conn
        .prepare("SELECT file FROM mods_install_files WHERE record_id = ?1")
        .map_err(|e| format!("数据库创建查询失败: {}", e))?;
    let files_iter = stmt
        .query_map([record_id], |row| Ok(row.get::<_, String>(0)?))
        .map_err(|e| format!("数据查询失败: {}", e))?;

    let files = files_iter.filter_map(Result::ok).collect();

    Ok(files)
}

/// 查询mod存档
#[tauri::command]
pub fn get_mod_records(mode_type: &str, search: &str) -> Result<Vec<GameMod>, String> {
    let conn = open_data_db().map_err(|e| format!("数据库连接失败: {}", e))?;

    let mut sql = String::from("SELECT * FROM mods_records WHERE mod_type = ?1 ORDER BY id ASC");
    let mut params: Vec<&dyn ToSql> = Vec::new();
    params.push(&mode_type);
    let like_query = format!("%{}%", search);

    if !search.is_empty() {
        sql = String::from(
            "SELECT * FROM mods_records WHERE mod_type = ?1 AND name LIKE ?2 ORDER BY id ASC",
        );
        params.push(&like_query);
    }

    let mut stmt = conn
        .prepare(&sql)
        .map_err(|e| format!("数据库创建查询失败: {}", e))?;
    let records_iter = stmt
        .query_map(params_from_iter(params), |row| {
            Ok(GameMod {
                id: row.get(0).ok(),
                name: row.get(1)?,
                memo: row.get(2)?,
                mod_type: row.get(3)?,
                activate: row.get(4)?,
                author: row.get(5)?,
                link: row.get(6)?,
                desc: row.get(7)?,
                preview: row.get(8)?,
            })
        })
        .map_err(|e| format!("数据查询失败: {}", e))?;

    let records = records_iter.filter_map(Result::ok).collect();

    Ok(records)
}

/// 获取所有安装文件
pub fn get_all_mods_install_files(mod_type: &str) -> Result<Vec<String>> {
    let conn = open_data_db().context("数据库连接失败")?;

    let mut stmt = conn
        .prepare("SELECT file FROM mods_install_files AS files LEFT JOIN mods_records AS record ON files.record_id = record.id WHERE record.mod_type = ?1").context("创建查询失败")?;
    let files_iter = stmt
        .query_map([mod_type], |row| Ok(row.get::<_, String>(0)?))
        .context("查询失败")?;

    let files = files_iter.filter_map(Result::ok).collect();

    Ok(files)
}

/// 更新mod存档安装状态
fn update_mod_record_activate(conn: &Connection, game_mod: &GameModData) -> Result<i64> {
    let id = game_mod.info.id.unwrap_or(0);
    conn.execute(
        "UPDATE mods_records SET activate = ?1 WHERE id = ?2",
        (u8::from(*(&game_mod.info.activate)), &id),
    )?;

    Ok(id)
}

/// 更新mod存档信息
pub fn update_mod_record_info(game_mod: &GameMod) -> Result<()> {
    let conn = open_data_db()?;
    let id = game_mod.id.unwrap_or(0);
    conn.execute(
        "UPDATE mods_records SET memo = ?1,mod_type = ?2,author = ?3,link = ?4,desc = ?5,preview = ?6 WHERE id = ?7",
        (&game_mod.memo, &game_mod.mod_type, &game_mod.author, &game_mod.link, &game_mod.desc, &game_mod.preview, &id),
    )?;

    Ok(())
}

/// 添加mod存档
fn add_mod_record(conn: &Connection, game_mod: &GameModData) -> Result<i64> {
    conn.execute(
        "INSERT INTO mods_records (name, memo, mod_type, activate, author, link, desc, preview) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        (&game_mod.info.name, &game_mod.info.name, &game_mod.info.mod_type, u8::from(*(&game_mod.info.activate)) , &game_mod.info.author, &game_mod.info.link, &game_mod.info.desc, &game_mod.info.preview),
    )?;
    let last_record_id = conn.last_insert_rowid();
    // println!("last_record_id: {last_record_id}");

    Ok(last_record_id)
}

/// 从数据库删除一个mod存档
pub fn del_one_mod_record(record_id: i64) -> Result<()> {
    let conn = open_data_db()?;

    conn.execute("DELETE FROM mods_records WHERE id = ?1", params![record_id])?;

    conn.execute(
        "DELETE FROM mods_install_files WHERE record_id = ?1",
        params![record_id],
    )?;

    Ok(())
}

/// 一键卸载：删除所有mods_install_files安装记录，mods_records所有activate置为0
pub fn uninstall_all_mod(mod_type: &str) -> Result<()> {
    let mut conn = open_data_db()?;
    let tx = conn.transaction()?;

    tx.execute(
        "UPDATE mods_records SET activate = 0 WHERE mod_type = ?1 ",
        params![mod_type],
    )?;

    tx.execute(
        "DELETE FROM mods_install_files WHERE record_id IN ( SELECT id FROM mods_records WHERE mod_type = ?1)", params![mod_type]
    )?;
    tx.commit()?;
    Ok(())
}

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

pub fn update_mod(game_mod: GameModData) -> Result<()> {
    let mut conn = open_data_db()?;
    let tx = conn.transaction()?;

    // 存档记录
    let record_id = update_mod_record_activate(&tx, &game_mod)?; // tx causes rollback if this fails

    // 安装记录
    if game_mod.info.activate {
        add_mod_install_files(&tx, &game_mod, record_id)?; // tx causes rollback if this fails
    }

    tx.commit()?;
    Ok(())
}

pub fn add_mod(game_mod: GameModData) -> Result<()> {
    let mut conn = open_data_db()?;
    let tx = conn.transaction()?;

    // 存档记录
    let record_id = add_mod_record(&tx, &game_mod)?; // tx causes rollback if this fails

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
            memo        TEXT NOT NULL,
            mod_type    TEXT NOT NULL,
            activate    INTEGER NOT NULL,
            author      TEXT,
            link        TEXT,
            desc        TEXT,
            preview     TEXT
        )",
        (),
    )?;

    // 已安装mods文件记录表，install_id用于表明哪些文件属于哪条记录
    conn.execute(
        "CREATE TABLE IF NOT EXISTS mods_install_files (
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
            // println!("Directory created");
        } else {
            // println!("Directory already exists");
        }

        // sqlite3的db文件路径=db_path
        db_path.push("data.db");
        // println!("{:?}", db_path);
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
