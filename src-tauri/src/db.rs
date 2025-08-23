use rusqlite::{Connection, OptionalExtension, ToSql, params, params_from_iter};
extern crate dirs;
use crate::config as my_config;
use crate::mods::{GameMod, GameModData, GameModWithEnv};
use anyhow::{Context, Result, anyhow};
use serde::Serialize;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

#[tauri::command]
pub fn db_operate_test() {}

pub fn check_mod_path(name: &str) -> Result<bool> {
    let conn = open_data_db()?;

    // let mut stmt = conn
    //     .prepare("SELECT COUNT(*) as count FROM mods_records WHERE path = ?1")
    //     .map_err(|e| format!("查询Mod路径是否存在失败: {}", e))?;
    // let count: u32 = stmt
    //     .query_row(params![name], |row| row.get(0))
    //     .map_err(|e| format!("查询Mod路径是否存在失败: {}", e))?;

    // // println!("count:{}", count);
    // if count == 0 { Ok(true) } else { Ok(false) }

    let path: Option<String> = conn
        .query_row(
            "SELECT path FROM mods_records WHERE path = ?1",
            [name],
            |row| row.get(0),
        )
        .optional()?;

    if !path.is_none() { Ok(true) } else { Ok(false) }
}

pub fn check_mod_install_status(record_id: u64) -> Result<bool> {
    let conn = open_data_db()?;
    let file: Option<String> = conn
        .query_row(
            "SELECT file FROM mods_install_files WHERE record_id = ?",
            [record_id],
            |row| row.get(0),
        )
        .optional()?;

    if !file.is_none() { Ok(true) } else { Ok(false) }
}

pub fn get_environment_mod_last_sort(env_id: u32) -> Result<Option<String>> {
    let conn = open_data_db()?;
    let sort: Option<String> = conn
        .query_row(
            "SELECT sort FROM environment_mods WHERE env_id = ? ORDER BY sort DESC LIMIT 1",
            [env_id],
            |row| row.get(0),
        )
        .optional()?;

    Ok(sort)
}

pub fn get_environment_install_mods_num(env_id: u32) -> Result<u64> {
    let conn = open_data_db()?;
    let num: u64 = conn.query_row(
        "SELECT count(*) as count FROM mods_install_files WHERE env_id = ? ",
        [env_id],
        |row| row.get(0),
    )?;

    Ok(num)
}

#[derive(Serialize)]
pub struct Statistic {
    records_activate_count: u32,
    records_total_count: u32,
    model_activate_count: u32,
    model_total_count: u32,
    voice_activate_count: u32,
    voice_total_count: u32,
}

#[allow(dead_code)]
#[tauri::command]
pub fn get_statistics() -> Result<Statistic, String> {
    let conn = open_data_db().map_err(|e| format!("数据库连接失败: {}", e))?;

    let mut stmt = conn
        .prepare(
            "SELECT 
            (SELECT COUNT(*) FROM mods_records WHERE activate=1),
            (SELECT COUNT(*) FROM mods_records),
            (SELECT COUNT(*) FROM mods_records WHERE type = 'model' AND activate=1),
            (SELECT COUNT(*) FROM mods_records WHERE type = 'model'),
            (SELECT COUNT(*) FROM mods_records WHERE type = 'voice' AND activate=1 ),
            (SELECT COUNT(*) FROM mods_records WHERE type = 'voice')
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

#[derive(Serialize, Debug)]
pub struct Environment {
    id: u32,
    name: String,
    activate: u32,
}

#[tauri::command]
pub fn get_environment_list() -> Result<Vec<Environment>, String> {
    let conn: Connection = open_data_db().map_err(|e| format!("数据库连接失败: {}", e))?;

    let mut stmt = conn
        .prepare("SELECT * FROM environment_lists")
        .map_err(|e| format!("数据库创建查询失败: {}", e))?;
    let env_iter = stmt
        .query_map([], |row| {
            Ok(Environment {
                id: row.get(0)?,
                name: row.get(1)?,
                activate: row.get(2)?,
            })
        })
        .map_err(|e| format!("数据查询失败: {}", e))?;

    let env = env_iter
        .inspect(|res| {
            if let Err(e) = res {
                eprintln!("get_environment_list Error: {}", e);
            }
        })
        .filter_map(Result::ok)
        .collect();

    Ok(env)
}

/// 更新环境状态
#[tauri::command(rename_all = "snake_case")]
pub fn update_environment_activate(id: u32) -> Result<(), String> {
    let conn = open_data_db().map_err(|e| format!("数据库连接失败: {}", e))?;

    conn.execute(
        "UPDATE environment_lists SET activate = CASE WHEN id = ? THEN 1 ELSE 0 END",
        params![id],
    )
    .map_err(|e| format!("更新失败: {}", e))?;

    Ok(())
}

/// 更新环境状态
#[tauri::command(rename_all = "snake_case")]
pub fn update_environment_name(id: u32, name: &str) -> Result<(), String> {
    let conn = open_data_db().map_err(|e| format!("数据库连接失败: {}", e))?;
    conn.execute(
        "UPDATE environment_lists SET name = ? WHERE id = ?",
        params![name, id],
    )
    .map_err(|e| format!("更新失败: {}", e))?;

    Ok(())
}

pub fn add_environment(name: &str) -> Result<()> {
    let conn = open_data_db()?;
    conn.execute(
        "INSERT INTO environment_lists (name, activate) VALUES (?, ?)",
        (name, 0),
    )?;

    Ok(())
}

pub fn copy_environment(id: u32, name: &str) -> Result<()> {
    let mut conn = open_data_db()?;
    let tx = conn.transaction()?;

    let name = format!("{}_copy", name);
    tx.execute(
        "INSERT INTO environment_lists (name, activate) VALUES (?, ?)",
        (&name, 0),
    )?;

    let last_env_id = tx.last_insert_rowid() as u32;
    // println!("copy_environment last_env_id: {}", last_env_id);

    tx.execute(
        "INSERT OR IGNORE INTO environment_mods (record_id, env_id, options, activate, sort)
        SELECT record_id, ?, options, activate, sort
        FROM environment_mods
        WHERE env_id = ?;",
        (last_env_id, id),
    )?;

    tx.commit()?;
    Ok(())
}

pub fn delete_environment(id: u32) -> Result<()> {
    let mut conn = open_data_db()?;
    let tx = conn.transaction()?;
    tx.execute("DELETE FROM environment_lists WHERE id = ?", params![id])?;
    tx.execute("DELETE FROM environment_mods WHERE env_id = ?", params![id])?;

    tx.commit()?;
    Ok(())
}

/// 查询该环境下此Mod的所有安装文件
pub fn get_env_mod_install_files(env_id: u32, record_id: u64) -> Result<Vec<String>> {
    let conn = open_data_db()?;

    let mut stmt =
        conn.prepare("SELECT file FROM mods_install_files WHERE record_id = ? AND env_id = ?")?;
    let files_iter = stmt.query_map((record_id, env_id), |row| Ok(row.get::<_, String>(0)?))?;

    let files = files_iter.filter_map(Result::ok).collect();

    Ok(files)
}

/// 查询该环境下添加的Mod列表
#[tauri::command(rename_all = "snake_case")]
pub fn get_environment_mods_records(
    env_id: u32,
    search: &str,
    order_asc_flag: u8,
) -> Result<Vec<GameMod>, String> {
    let conn = open_data_db().map_err(|e| format!("数据库连接失败: {}", e))?;

    let mut sql = String::from(
        "SELECT m.id, m.uuid, m.name, m.path, m.tag, m.author, m.link, m.desc, m.icon, e.sort, m.type, m.version, e.id as env_mod_id, e.activate,
            CASE WHEN EXISTS (
                SELECT 1 FROM mods_install_files f
                WHERE f.record_id = e.record_id AND f.env_id = ?
            ) THEN 1 ELSE 0 END AS env_mod_install_flag 
            FROM environment_mods e 
            LEFT JOIN mods_records m ON m.id = e.record_id 
            WHERE e.env_id = ? ",
    );
    let mut params: Vec<&dyn ToSql> = Vec::new();
    params.push(&env_id);
    params.push(&env_id);

    let like_query = format!("%{}%", search);
    if !search.is_empty() {
        sql += " AND m.name LIKE ?";
        params.push(&like_query);
    }

    if order_asc_flag == 1 {
        sql += " ORDER BY e.sort ASC";
    } else {
        sql += " ORDER BY e.sort DESC";
    }

    // println!("get_environment_mods_records: {sql}");

    let mut stmt = conn
        .prepare(&sql)
        .map_err(|e| format!("数据库创建查询失败: {}", e))?;
    let records_iter = stmt
        .query_map(params_from_iter(params), |row| {
            Ok(GameMod {
                id: row.get("id")?,
                uuid: row.get("uuid")?,
                name: row.get("name")?,
                path: row.get("path")?,
                tag: row.get("tag")?,
                author: row.get("author")?,
                link: row.get("link")?,
                desc: row.get("desc")?,
                icon: row.get("icon")?,
                sort: row.get("sort")?,
                r#type: row.get("type")?,
                version: row.get("version")?,
                options: String::new(),
                activate: row.get("activate")?,
                env_mod_id: row.get("env_mod_id").ok(),
                env_mod_options: None,
                env_mod_install_flag: row.get("env_mod_install_flag").ok(),
            })
        })
        .map_err(|e| format!("数据查询失败: {}", e))?;

    let records = records_iter
        .inspect(|res| {
            if let Err(e) = res {
                eprintln!("get_environment_mods_records Error: {}", e);
            }
        })
        .filter_map(Result::ok)
        .collect();
    // println!("records: {:?}", records);

    Ok(records)
}

/// 查询mod存档
#[tauri::command(rename_all = "snake_case")]
pub fn get_mods_records_with_env_add_flag(env_id: u32) -> Result<Vec<GameModWithEnv>, String> {
    let conn = open_data_db().map_err(|e| format!("数据库连接失败: {}", e))?;

    let sql = String::from(
        "SELECT m.id, m.name, m.path, m.icon,
            CASE WHEN EXISTS (
                SELECT 1 FROM environment_mods e
                WHERE e.record_id = m.id AND e.env_id = ?
            ) THEN 1 ELSE 0 END AS env_add_flag 
            FROM mods_records m",
    );
    let mut params: Vec<&dyn ToSql> = Vec::new();
    params.push(&env_id);

    let mut stmt = conn
        .prepare(&sql)
        .map_err(|e| format!("数据库创建查询失败: {}", e))?;
    let records_iter = stmt
        .query_map(params_from_iter(params), |row| {
            Ok(GameModWithEnv {
                id: row.get("id")?,
                name: row.get("name")?,
                path: row.get("path")?,
                icon: row.get("icon")?,
                env_add_flag: row.get("env_add_flag")?,
            })
        })
        .map_err(|e| format!("数据查询失败: {}", e))?;

    let records = records_iter
        .inspect(|res| {
            if let Err(e) = res {
                eprintln!("get_mods_records_with_env_add_flag Error: {}", e);
            }
        })
        .filter_map(Result::ok)
        .collect();
    // println!("records: {:?}", records);

    Ok(records)
}

pub fn get_mods_records_by_ids(ids: Vec<u64>) -> Result<Vec<GameMod>> {
    let conn = open_data_db()?;
    let placeholders = ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
    let sql = format!("SELECT * FROM mods_records WHERE id IN ({})", placeholders);

    let mut stmt = conn.prepare(&sql)?;
    let records_iter = stmt.query_map(params_from_iter(ids), |row| {
        Ok(GameMod {
            id: row.get("id")?,
            uuid: row.get("uuid")?,
            name: row.get("name")?,
            path: row.get("path")?,
            tag: row.get("tag")?,
            author: row.get("author")?,
            link: row.get("link")?,
            desc: row.get("desc")?,
            icon: row.get("icon")?,
            sort: String::new(),
            r#type: row.get("type")?,
            version: row.get("version")?,
            options: row.get("options")?,
            activate: 0,
            env_mod_id: None,
            env_mod_options: None,
            env_mod_install_flag: None,
        })
    })?;

    let records = records_iter
        .inspect(|res| {
            if let Err(e) = res {
                eprintln!("get_mods_records_by_ids Error: {}", e);
            }
        })
        .filter_map(Result::ok)
        .collect();
    // println!("records: {:?}", records);

    Ok(records)
}

pub fn add_mods_to_environment(env_id: u32, mods: Vec<GameMod>, sorts: Vec<String>) -> Result<()> {
    let mut conn = open_data_db()?;
    let tx = conn.transaction()?;
    {
        // 预编译 SQL
        let mut stmt = tx.prepare("INSERT OR IGNORE INTO environment_mods (record_id, env_id, options, activate, sort) VALUES (?, ?, ?, ?, ?)")?;

        for (i, gm) in mods.iter().enumerate() {
            stmt.execute(rusqlite::params![gm.id, env_id, gm.options, 0, sorts[i]])?;
        }
    } // stmt 在这里 drop，tx 不再被借用
    tx.commit()?;

    Ok(())
}

/// 查询mod详情
pub fn get_env_mod_info(id: u64, env_id: u32) -> Result<GameMod> {
    let conn = open_data_db()?;
    let record = conn
        .query_row(
            "SELECT m.id, m.uuid, m.name, m.path, m.tag, m.author, m.link, m.desc, m.icon, e.sort, m.type, m.version, e.id as env_mod_id, e.activate, e.options as env_mod_options,
                    CASE WHEN EXISTS (
                        SELECT 1 FROM mods_install_files f
                        WHERE f.record_id = e.record_id AND f.env_id = ?
                    ) THEN 1 ELSE 0 END AS env_mod_install_flag 
                FROM environment_mods e LEFT JOIN mods_records m ON m.id = e.record_id 
                WHERE m.id = ? AND e.env_id = ? ",
            params![env_id, id, env_id],
            |row| {
                Ok(GameMod {
                    id: row.get("id")?,
                    uuid: row.get("uuid")?,
                    name: row.get("name")?,
                    path: row.get("path")?,
                    tag: row.get("tag")?,
                    author: row.get("author")?,
                    link: row.get("link")?,
                    desc: row.get("desc")?,
                    icon: row.get("icon")?,
                    sort: row.get("sort")?,
                    r#type: row.get("type")?,
                    version: row.get("version")?,
                    options: String::new(),
                    activate: row.get("activate")?,
                    env_mod_id: row.get("env_mod_id").ok(),
                    env_mod_options : row.get("env_mod_options").ok(),
                    env_mod_install_flag: row.get("env_mod_install_flag").ok(),
                })
            },
        )?;
    Ok(record)
}

/// 查询已激活的环境Mods
pub fn get_environment_activate_mods(
    env_id: u32,
    mods_install_priority: &str,
) -> anyhow::Result<Vec<GameMod>> {
    let conn = open_data_db()?;

    let mut sql = String::from(
        "SELECT m.id, m.name, m.path, m.type, e.id as env_mod_id, e.options as env_mod_options 
        FROM environment_mods e 
        LEFT JOIN mods_records m ON e.record_id = m.id 
        WHERE e.env_id = ? AND e.activate = 1 
        ORDER BY e.sort ",
    );

    sql += mods_install_priority;

    let mut stmt = conn.prepare(&sql)?;
    let mods_iter = stmt.query_map([env_id], |row| {
        Ok(GameMod {
            id: row.get("id")?,
            name: row.get("name")?,
            path: row.get("path")?,
            r#type: row.get("type")?,
            activate: 1,
            env_mod_id: row.get("env_mod_id").ok(),
            env_mod_options: row.get("env_mod_options").ok(),
            ..Default::default()
        })
    })?;

    let mods = mods_iter
        .inspect(|res| {
            if let Err(e) = res {
                eprintln!("get_environment_activate_mods Error: {}", e);
            }
        })
        .filter_map(Result::ok)
        .collect();

    Ok(mods)
}

/// 获取所有安装文件
pub fn get_all_mods_install_files() -> Result<Vec<String>> {
    let conn = open_data_db().context("数据库连接失败")?;

    let mut stmt = conn
        .prepare("SELECT file FROM mods_install_files")
        .context("创建查询失败")?;
    let files_iter = stmt
        .query_map([], |row| Ok(row.get::<_, String>(0)?))
        .context("查询失败")?;

    let files = files_iter.filter_map(Result::ok).collect();

    Ok(files)
}

/// 更新mod存档安装状态
// fn update_mod_record_activate(conn: &Connection, game_mod: &GameModData) -> Result<u64> {
//     let id = game_mod.info.id;
//     conn.execute(
//         "UPDATE mods_records SET activate = ?1 WHERE id = ?2",
//         (&game_mod.info.activate, &id),
//     )?;

//     Ok(id)
// }

/// 更新环境mod安装状态
#[tauri::command(rename_all = "snake_case")]
pub fn update_environment_mod_activate(id: u64, activate: u8) -> Result<(), String> {
    let conn = open_data_db().map_err(|e| format!("数据库连接失败: {}", e))?;

    conn.execute(
        "UPDATE environment_mods SET activate = ? WHERE id = ?",
        (activate, id),
    )
    .map_err(|e| format!("更新失败: {}", e))?;

    Ok(())
}

/// 更新环境mod配置
#[tauri::command(rename_all = "snake_case")]
pub fn update_environment_mod_options(id: u64, options: &str) -> Result<(), String> {
    let conn = open_data_db().map_err(|e| format!("数据库连接失败: {}", e))?;

    conn.execute(
        "UPDATE environment_mods SET options = ? WHERE id = ?",
        (options, id),
    )
    .map_err(|e| format!("更新失败: {}", e))?;

    Ok(())
}

/// 更新环境mod排序值
pub fn update_environment_mod_sort(id: u64, sort: &str) -> Result<(), String> {
    let conn = open_data_db().map_err(|e| format!("数据库连接失败: {}", e))?;

    conn.execute(
        "UPDATE environment_mods SET sort = ? WHERE id = ?",
        (sort, id),
    )
    .map_err(|e| format!("更新失败: {}", e))?;

    Ok(())
}

/// 更新mod存档信息
pub fn update_mod_record_info(game_mod: &GameMod) -> Result<()> {
    let conn = open_data_db()?;
    let id = game_mod.id;
    conn.execute(
        "UPDATE mods_records SET name = ?, version = ?, author = ?,link = ?,desc = ?,icon = ? WHERE id = ?",
        (&game_mod.name, &game_mod.version, &game_mod.author, &game_mod.link, &game_mod.desc, &game_mod.icon, id),
    )?;

    Ok(())
}

/// 添加mod存档
fn add_mod_record(conn: &Connection, game_mod: &GameModData) -> Result<u64> {
    let add_time = current_timestamp("ms")?;
    conn.execute(
        "INSERT INTO mods_records (uuid, name, path, type, author, link, desc, icon, version, options, add_time) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        (&game_mod.info.uuid, &game_mod.info.name, &game_mod.info.path, &game_mod.info.r#type, &game_mod.info.author, &game_mod.info.link, &game_mod.info.desc, &game_mod.info.icon, &game_mod.info.version, &game_mod.info.options, add_time),
    )?;
    let last_record_id = conn.last_insert_rowid() as u64;
    // println!("last_record_id: {last_record_id}");

    Ok(last_record_id)
}

fn current_timestamp(unit: &str) -> Result<u64> {
    let now = SystemTime::now();
    let duration = now.duration_since(UNIX_EPOCH)?;

    match unit {
        "s" => Ok(duration.as_secs()),
        "ms" => Ok(duration.as_millis() as u64),
        "us" => Ok(duration.as_micros() as u64),
        _ => Ok(duration.as_millis() as u64),
    }
}

fn add_environment_mod(
    conn: &Connection,
    record_id: u64,
    env_id: u32,
    activate: u8,
    options: &str,
    sort: &str,
) -> Result<u64> {
    conn.execute(
        "INSERT INTO environment_mods (record_id, env_id, options, activate, sort) VALUES (?, ?, ?, ?, ?)",
        (record_id, env_id, options, activate, sort),
    )?;
    let last_env_mod_id = conn.last_insert_rowid() as u64;
    // println!("last_env_mod_id: {last_env_mod_id}");

    Ok(last_env_mod_id)
}

/// 删除一个mod
pub fn delete_one_mod(record_id: u64, env_id: u32, delete_file_flag: u8) -> Result<()> {
    let mut conn = open_data_db()?;
    let tx = conn.transaction()?;
    if delete_file_flag == 0 {
        // 只删除环境Mod记录
        tx.execute(
            "DELETE FROM environment_mods WHERE record_id = ? AND env_id = ?",
            params![record_id, env_id],
        )?;
    } else {
        // 删除所有记录
        tx.execute(
            "DELETE FROM environment_mods WHERE record_id = ?",
            params![record_id],
        )?;

        tx.execute("DELETE FROM mods_records WHERE id = ?", params![record_id])?;
    }
    tx.commit()?;
    Ok(())
}

/// 一键卸载：删除所有mods_install_files安装记录
pub fn uninstall_all_mods() -> Result<()> {
    let mut conn = open_data_db()?;
    let tx = conn.transaction()?;

    // 无需改变环境Mod的activate状态，以方便重新安装（install_mods接口调用需要根据activate状态进行安装
    // tx.execute("UPDATE environment_mods SET activate = 0;", params![])?;

    tx.execute("DELETE FROM mods_install_files;", params![])?;
    tx.commit()?;
    Ok(())
}

/// 记录mod所安装的文件
pub fn add_mod_install_files(record_id: u64, env_id: u32, files: &Vec<String>) -> Result<()> {
    let mut conn = open_data_db()?;
    let tx = conn.transaction()?;
    for file in files {
        tx.execute(
            "INSERT INTO mods_install_files (record_id, env_id, file ) VALUES (?, ?, ?)",
            (record_id, env_id, file),
        )?;
    }
    tx.commit()?;
    Ok(())
}

#[allow(dead_code)]
pub fn update_mod(_game_mod: GameModData) -> Result<()> {
    let mut conn = open_data_db()?;
    let tx = conn.transaction()?;

    // 存档记录
    // let record_id = update_mod_record_activate(&tx, &game_mod)?;

    // 安装记录
    // if game_mod.info.activate == 1 {
    //     add_mod_install_files(&tx, &game_mod, record_id)?;
    // }

    tx.commit()?;
    Ok(())
}

pub fn add_mod(game_mod: GameModData, env_id: u32) -> Result<()> {
    let mut conn = open_data_db()?;
    let tx = conn.transaction()?;

    // 存档记录
    let record_id = add_mod_record(&tx, &game_mod)?;

    let _ = add_environment_mod(
        &tx,
        record_id,
        env_id,
        game_mod.info.activate,
        &game_mod.info.options,
        &game_mod.info.sort,
    )?;

    // 安装记录
    // if game_mod.info.activate == 1 {
    //     add_mod_install_files(&tx, &game_mod, record_id)?;
    // }

    tx.commit()?;
    Ok(())
}

pub fn init_database() -> tauri::Result<()> {
    create_db_table_if_not_exists()?;
    println!("init_database数据库初始化！");
    Ok(())
}

pub fn create_db_table_if_not_exists() -> Result<()> {
    let conn = open_data_db()?;

    // mods存档，包含：名称、类型、安装状态、作者、链接、详情、预览图
    conn.execute(
        "CREATE TABLE IF NOT EXISTS mods_records (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            uuid        TEXT NOT NULL DEFAULT '',
            name        TEXT NOT NULL,
            path        TEXT NOT NULL,
            tag         TEXT NOT NULL DEFAULT '',
            author      TEXT NOT NULL DEFAULT '',
            link        TEXT NOT NULL DEFAULT '',
            desc        TEXT NOT NULL DEFAULT '',
            icon        TEXT NOT NULL DEFAULT '',
            type        INTEGER NOT NULL,
            options     TEXT NOT NULL DEFAULT '',
            version     TEXT NOT NULL DEFAULT '',
            add_time    INTEGER NOT NULL DEFAULT 0,
            up_time    INTEGER NOT NULL DEFAULT 0
        )",
        (),
    )?;
    conn.execute(
        "CREATE UNIQUE INDEX IF NOT EXISTS unique_mod_path ON mods_records(path);",
        (),
    )?;

    // 游戏已安装mod文件记录，可查看游戏里的文件对应哪个mod；record_id关联mods_records表的id
    conn.execute(
        "CREATE TABLE IF NOT EXISTS mods_install_files (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            record_id   INTEGER NOT NULL,
            env_id      INTEGER NOT NULL,
            file        TEXT NOT NULL

    )",
        (),
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_install_env_mod ON mods_install_files(record_id, env_id)",
        (),
    )?;

    // 环境切换：用于快速切换不同环境下的mod状态
    conn.execute(
        "CREATE TABLE IF NOT EXISTS environment_mods (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            record_id   INTEGER NOT NULL,
            env_id      INTEGER NOT NULL,
            options     TEXT NOT NULL DEFAULT '',
            activate    INTEGER NOT NULL DEFAULT 0,
            sort        TEXT NOT NULL DEFAULT ''
        )",
        (),
    )?;
    conn.execute(
        "CREATE UNIQUE INDEX IF NOT EXISTS unique_env_mod ON environment_mods(record_id, env_id)",
        (),
    )?;

    // 环境列表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS environment_lists (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            name        TEXT NOT NULL,
            activate    INTEGER NOT NULL
        )",
        (),
    )?;
    conn.execute(
        "CREATE UNIQUE INDEX IF NOT EXISTS unique_env_name ON environment_lists(name);",
        (),
    )?;
    conn.execute(
        "INSERT OR IGNORE INTO environment_lists(name, activate) VALUES('default', 1);",
        (),
    )?;

    Ok(())
}

pub fn open_data_db() -> Result<Connection> {
    // v1 com.luckyriko.helldivers2
    // v2 com.luckyriko.hellrat

    let app_identifier = my_config::get_identifier().unwrap_or("com.luckyriko.hellrat".into());
    // println!("db.rs: app_identifier值是 {}", app_identifier);

    if let Some(mut db_path) = dirs::config_dir() {
        db_path.push(app_identifier);
        db_path.push("db");

        if !db_path.exists() {
            fs::create_dir_all(&db_path)?;
        }

        // sqlite3的db文件路径
        db_path.push("data.db");
        // println!("{:?}", db_path);

        let path = db_path.to_string_lossy().to_string();
        let conn = Connection::open(path).context("Failed to open data.db")?;

        Ok(conn)
    } else {
        Err(anyhow!("获取用户config_dir失败"))
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
