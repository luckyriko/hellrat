use crate::config as my_config;
use crate::db as my_db;
use crate::fs as my_fs;
use json5;
use mudder::SymbolTable;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use tauri::{AppHandle, Emitter};

#[derive(Serialize, Deserialize, Debug)]
pub struct GameMod {
    pub id: u64,
    pub uuid: String,
    pub name: String,
    pub path: String,
    pub tag: String,
    pub author: String,
    pub link: String,
    pub desc: String,
    pub icon: String,
    pub sort: String,
    pub r#type: u8,
    pub version: String,
    pub options: String,
    pub activate: u8,
    pub env_mod_id: Option<u64>,
    pub env_mod_options: Option<String>,
    pub env_mod_install_flag: Option<u8>,
}

impl Default for GameMod {
    fn default() -> Self {
        Self {
            id: 0,
            uuid: String::new(),
            name: String::new(),
            path: String::new(),
            tag: String::new(),
            author: String::new(),
            link: String::new(),
            desc: String::new(),
            icon: String::new(),
            sort: String::new(),
            r#type: 1,
            version: String::new(),
            options: String::new(),
            activate: 0,
            env_mod_id: None,
            env_mod_options: None,
            env_mod_install_flag: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameModWithEnv {
    pub id: u64,
    pub name: String,
    pub path: String,
    pub icon: String,
    pub env_add_flag: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameModData {
    pub info: GameMod,
    pub files: Vec<String>,
}

// https://github.com/teutinsa/Helldivers2ModManager/blob/master/mod_manifest_v1-schema.json
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct Manifest {
    Version: u64,
    Guid: String,
    Name: String,
    Description: String,
    IconPath: Option<String>,
    Options: Option<Vec<ManifestOptions>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct ManifestOptions {
    Name: String,
    Description: String,
    Image: Option<String>,
    Include: Option<Vec<String>>,
    SubOptions: Option<Vec<ManifestSubOptions>>,
    Activate: Option<bool>,
    SubOptionValue: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct ManifestSubOptions {
    Name: String,
    Description: String,
    Image: Option<String>,
    Include: Vec<String>,
}

// Mod存档（不加async会造成ui卡顿、无响应
#[tauri::command(async, rename_all = "snake_case")]
pub fn save_mods(app: AppHandle, paths: Vec<String>, env_id: u32) -> Result<(), String> {
    // 获取本地配置的目录
    let config_json = my_config::get_app_config_json(&app, "game_mod").unwrap_or(json!({}));
    let game_data_path = config_json
        .get("game_data_path")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let mods_store_path = config_json
        .get("mods_store_path")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let mods_temp_cache_path = config_json
        .get("mods_temp_cache_path")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    if game_data_path.is_empty() || mods_store_path.is_empty() || mods_temp_cache_path.is_empty() {
        return Err(format!("获取本地配置目录失败"));
    }

    if paths.len() <= 0 {
        return Err(format!("请选择至少一个文件或目录"));
    }

    let env_mod_sorts = get_lexorank_next_value(env_id, paths.len())
        .map_err(|e| format!("获取排序值失败: {}", e))?;

    for (i, item) in paths.iter().enumerate() {
        println!("第 {} 个数是 {}", i, item);

        // Global events are delivered to all listeners
        let progress = json!({
            "index": i,
            "status": "start",
        });
        app.emit("save-mods-progress", progress.to_string())
            .map_err(|e| format!("投递信息失败: {}", e))?;

        let path = Path::new(item);
        let file_extension = path
            .extension()
            .map(|os| os.to_string_lossy().to_string())
            .unwrap_or_default();

        let file_stem = path
            .file_stem()
            .map(|os| os.to_string_lossy().to_string())
            .unwrap_or_default();

        let dest_path_buf = Path::new(mods_store_path).join(&file_stem);
        // println!("{:?}", dest_path_buf);

        let already_install_flag = my_db::check_mod_path(&file_stem)
            .map_err(|e| format!("检测到check_mod_path时发生了错误: {}", e))?;

        println!("{} 安装状态：{}", &file_stem, already_install_flag);
        if already_install_flag {
            println!("已经安装过了/路径名称重复，跳过。");

            let progress = json!({
                "index": i,
                "status": "re-path",
            });
            app.emit("save-mods-progress", progress.to_string())
                .map_err(|e| format!("投递信息失败: {}", e))?;
            continue;
        }

        if path.is_dir() {
            println!("是目录");
            let progress = json!({
                "index": i,
                "status": "copy",
            });
            app.emit("save-mods-progress", progress.to_string())
                .map_err(|e| format!("投递信息失败: {}", e))?;

            // 去复制到指定目录
            my_fs::dir_copy(path, &dest_path_buf).map_err(|e| format!("复制目录失败: {}", e))?;
        } else if path.is_file()
            && (file_extension == "zip" || file_extension == "7z" || file_extension == "rar")
        {
            println!("是zip文件");
            let progress = json!({
                "index": i,
                "status": "unzip",
            });
            app.emit("save-mods-progress", progress.to_string())
                .map_err(|e| format!("投递信息失败: {}", e))?;

            // 去解压
            let dest_path = dest_path_buf.to_string_lossy().to_string();
            if file_extension == "zip" {
                my_fs::unzip_one_file(item, &dest_path)
                    .map_err(|e| format!("解压文件失败: {}", e))?;
            } else if file_extension == "7z" {
                my_fs::un7z_one_file(item, &dest_path)
                    .map_err(|e| format!("解压文件失败: {}", e))?;
            } else if file_extension == "rar" {
                my_fs::unrar_one_file(item, &dest_path)
                    .map_err(|e| format!("解压文件失败: {}", e))?;
            }

            println!("解压zip文件完成");
        } else {
            println!("既不是目录也不是压缩文件");
            let progress = json!({
                "index": i,
                "status": "non-support",
            });
            app.emit("save-mods-progress", progress.to_string())
                .map_err(|e| format!("投递信息失败: {}", e))?;
            // 通知前端该文件格式无法解析
            // return Err(format!(
            //     "不支持的文件格式，既不是目录也不是zip、7z、rar压缩文件"
            // ));
            continue;
        }

        let progress = json!({
            "index": i,
            "status": "save",
        });
        app.emit("save-mods-progress", progress.to_string())
            .map_err(|e| format!("投递信息失败: {}", e))?;

        // 优化单目录结构
        let one_dir_flag = my_fs::has_only_one_directory(&dest_path_buf)
            .map_err(|e| format!("检测到has_only_one_directory时发生了错误: {}", e))?;
        if one_dir_flag {
            println!("优化单目录结构");
            my_fs::move_first_subdir_contents_to_parent(&dest_path_buf).map_err(|e| {
                format!(
                    "检测到move_first_subdir_contents_to_parent时发生了错误: {}",
                    e
                )
            })?;
        }

        // 添加到数据库进行记录
        // 如果有manifest.json则按其解析；没有则进行判断：有多个目录的生成manifest.json，只有文件的则不生成直接解析
        let mod_manifest_json = get_mod_manifest_json(&dest_path_buf)
            .map_err(|e| format!("检测到get_mod_manifest_json时发生了错误: {}", e))?;
        if let Some(mmj) = mod_manifest_json {
            println!("自带manifest_json");

            let mut r#type: u8 = 2;
            if mmj.Options.is_none() {
                r#type = 1;
            }

            let mod_info: GameMod = GameMod {
                uuid: mmj.Guid,
                name: mmj.Name,
                path: file_stem.clone(),
                version: mmj.Version.to_string(),
                desc: mmj.Description,
                icon: mmj.IconPath.unwrap_or_default(),
                r#type,
                activate: 0,
                options: mmj.Options.map_or("".to_string(), |v| {
                    serde_json::to_string(&v).unwrap_or_default()
                }),
                sort: env_mod_sorts[i].clone(),
                ..Default::default()
            };

            let game_mod = GameModData {
                info: mod_info,
                files: vec![],
            };

            my_db::add_mod(game_mod, env_id)
                .map_err(|e| format!("已添加，但生成添加记录失败。错误原因: {}", e))?;
        } else {
            // 检测有没有目录文件，如果有生成options配置项

            // 取第一个图片作为图标
            let mod_icon = my_fs::get_mod_first_image_name(&dest_path_buf);

            let dirs =
                has_dir(&dest_path_buf).map_err(|e| format!("检测是否存在目录失败: {}", e))?;
            if dirs.len() > 0 {
                // 生成options的Vec
                let mut options: Vec<ManifestOptions> = vec![];
                for dir_name in &dirs {
                    let dir_path = dest_path_buf.join(&dir_name);

                    // 目录下是否有子目录
                    let sub_dirs =
                        has_dir(&dir_path).map_err(|e| format!("检测是否存在子目录失败: {}", e))?;
                    let mut sub_options: Vec<ManifestSubOptions> = vec![];
                    for sub_dir_name in &sub_dirs {
                        let include_path = Path::new(dir_name).join(sub_dir_name);
                        sub_options.push(ManifestSubOptions {
                            Name: sub_dir_name.clone(),
                            Description: String::new(),
                            Image: Some(String::new()),
                            Include: vec![include_path.to_string_lossy().to_string()],
                        })
                    }

                    // 目录
                    options.push(ManifestOptions {
                        Name: dir_name.clone(),
                        Description: String::new(),
                        Image: Some(String::new()),
                        Include: Some(vec![dir_name.clone()]),
                        SubOptions: Some(sub_options),
                        Activate: Some(false),
                        SubOptionValue: Some(0),
                    })
                }

                let mod_info: GameMod = GameMod {
                    name: file_stem.clone(),
                    path: file_stem.clone(),
                    icon: mod_icon,
                    r#type: 2,
                    activate: 0,
                    options: serde_json::to_string(&options).unwrap_or(String::new()),
                    sort: env_mod_sorts[i].clone(),
                    ..Default::default()
                };

                let game_mod = GameModData {
                    info: mod_info,
                    files: vec![],
                };

                my_db::add_mod(game_mod, env_id)
                    .map_err(|e| format!("已添加到游戏，但生成添加记录失败: {}", e))?;

                println!("系统生成manifest_json");
            } else {
                let mod_info: GameMod = GameMod {
                    name: file_stem.clone(),
                    path: file_stem.clone(),
                    icon: mod_icon,
                    r#type: 1,
                    activate: 0,
                    sort: env_mod_sorts[i].clone(),
                    ..Default::default()
                };

                let game_mod = GameModData {
                    info: mod_info,
                    files: vec![],
                };

                my_db::add_mod(game_mod, env_id)
                    .map_err(|e| format!("已添加到游戏，但生成添加记录失败: {}", e))?;

                println!("没有manifest_json");
            }
        }

        let progress = json!({
            "index": i,
            "status": "end",
        });
        app.emit("save-mods-progress", progress.to_string())
            .map_err(|e| format!("投递信息失败: {}", e))?;
    }

    app.emit("save-mods-complete", 666)
        .map_err(|e| format!("投递信息失败: {}", e))?;

    Ok(())
}

#[tauri::command(async, rename_all = "snake_case")]
pub fn install_mods(app: AppHandle, env_id: u32) -> Result<(), String> {
    // 获取本地配置的目录
    let config_json = my_config::get_app_config_json(&app, "game_mod").unwrap_or(json!({}));
    let game_data_path = config_json
        .get("game_data_path")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let mods_store_path = config_json
        .get("mods_store_path")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let mods_install_priority = config_json
        .get("mods_install_priority")
        .and_then(|v| v.as_str())
        .unwrap_or("asc");

    if game_data_path.is_empty() || mods_store_path.is_empty() {
        return Err(format!("游戏、存档目录未配置"));
    }
    let mods_store_path = Path::new(mods_store_path);
    let game_data_path = Path::new(game_data_path);

    // 先卸载通过本软件安装的所有mod
    uninstall_db_mods(game_data_path).map_err(|e| format!("Mod卸载失败：{}", e))?;

    // 拉取要安装的mod列表，如果是没有options则直接安装，否则需要解析options进行安装
    let mods = my_db::get_environment_activate_mods(env_id, mods_install_priority)
        .map_err(|e| format!("获取已安装Mod失败: {}", e))?;

    if mods.len() == 0 {
        return Err(format!("没有Mod将要被安装"));
    }

    for item in &mods {
        println!("----------{}-{}----------", item.name, item.r#type);

        let mut mod_install_files: Vec<String> = vec![];
        let mod_path = mods_store_path.join(&item.path);
        if item.r#type == 1 {
            // 直接安装
            mod_install_files = copy_mod_files_to_game_data_dir(&mod_path, game_data_path)
                .map_err(|e| format!("安装失败1-直接安装: {}", e))?;
        } else if item.r#type == 2 {
            // 根据配置文件json安装
            let mut options: Vec<ManifestOptions> = vec![];

            // item.env_mod_options字段不为空才解析
            if let Some(env_mod_options) = &item.env_mod_options {
                if !env_mod_options.is_empty() {
                    options = json5::from_str(env_mod_options)
                        .map_err(|e| format!("安装失败2-Json解析失败: {}", e))?;
                }
            }

            // 如果配置options.json是空数组则安装Mod根目录下的文件，否则按照options安装
            if options.is_empty() {
                mod_install_files = copy_mod_files_to_game_data_dir(&mod_path, game_data_path)
                    .map_err(|e| format!("安装失败2-根目录模式安装失败: {}", e))?;
            } else {
                mod_install_files =
                    copy_mod_files_to_game_data_dir_by_options(&mod_path, game_data_path, &options)
                        .map_err(|e| format!("安装失败2-Options模式安装失败: {}", e))?;
            }
        }

        my_db::add_mod_install_files(item.id, env_id, &mod_install_files)
            .map_err(|e| format!("安装成功但生成安装记录失败: {}", e))?;
        println!("----------{}安装文件：{:#?}", item.name, mod_install_files);
    }

    Ok(())
}

/// 获取游戏data目录最大的number补丁值
fn get_game_data_dir_max_number(prefix: &str, game_data_path: &Path) -> anyhow::Result<i32> {
    let mut max_number = -1;
    let patch_prefix = format!("{}.patch_", prefix);

    for entry in fs::read_dir(game_data_path)? {
        let entry = entry?;
        let tmp_var = entry.file_name();
        let file_name = tmp_var.to_string_lossy();

        // strip_prefix去掉匹配的前缀；number_str.parse::<i32>()尝试解析字符串为i32
        if let Some(number_str) = file_name.strip_prefix(&patch_prefix) {
            if let Ok(number) = number_str.parse::<i32>() {
                // println!("{} 排序数字: {}", prefix, number);
                max_number = max_number.max(number);
            } else {
                // 如果不是字符串形式的i32整数会解析失败，比如无法解析字符串+数字
            }
        }
    }

    println!("{} 最大排序数字: {}", prefix, max_number);
    Ok(max_number)
}

// 获取补丁前缀名称
fn extract_name(file_name: &str) -> &str {
    if let Some((name, suffix)) = file_name.split_once(".patch_") {
        if suffix.chars().all(|c| c.is_numeric()) {
            return name;
        }
    }
    ""
}

#[allow(dead_code)]
fn is_not_image(file_name: &str) -> bool {
    let image_extensions = [
        "jpg", "jpeg", "png", "gif", "bmp", "webp", "tiff", "ico", "svg", "jfif",
    ];

    match Path::new(file_name).extension() {
        Some(ext) => !image_extensions.contains(&ext.to_string_lossy().to_lowercase().as_str()),
        None => true, // 没有扩展名的文件也视为不是图片
    }
}

// 存在则返回所有目录名称，否则返回None
fn has_dir(path: &Path) -> anyhow::Result<Vec<String>> {
    let mut dir: Vec<String> = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        if metadata.is_dir() {
            let dir_name = entry.file_name().to_string_lossy().to_string();
            dir.push(dir_name);
        }
    }
    Ok(dir)
}

// 解析配置文件进行安装
// 先判断activate状态，如果为开启状态1，则先安装Include下的文件（activate字段可能不存在，在mod第一次加入且未编辑过的状态下）
// 如果有SubOptions，则取SubOptionValue值进行安装（根据值查找子目录进行安装）
fn copy_mod_files_to_game_data_dir_by_options(
    mod_path: &Path,
    game_data_path: &Path,
    options: &Vec<ManifestOptions>,
) -> anyhow::Result<Vec<String>> {
    let mut processed_files = vec![];

    for op in options {
        let activate = op.Activate.unwrap_or(false);

        if activate {
            // 先安装Include里的Mod
            let mod_include = op.Include.clone().unwrap_or_else(Vec::new);
            if mod_include.len() > 0 {
                println!("前置依赖将要被安装: {:#?}", mod_include);
                for include in mod_include {
                    let mut files =
                        copy_mod_files_to_game_data_dir(&mod_path.join(include), game_data_path)?;
                    processed_files.append(&mut files);
                }
            }
            // 再安装SubOptions里的Mod
            if let Some(sub_option_index) = op.SubOptionValue {
                if let Some(sub_options) = &op.SubOptions {
                    if sub_options.len() > 0 {
                        if let Some(sub_op) = sub_options.get(sub_option_index as usize) {
                            println!("sub_options选择的Include将要被安装: {:#?}", &sub_op.Include);
                            for include in &sub_op.Include {
                                let mut files = copy_mod_files_to_game_data_dir(
                                    &mod_path.join(include),
                                    game_data_path,
                                )?;
                                processed_files.append(&mut files);
                            }
                        } else {
                            println!("sub_options下标越界");
                        }
                    }
                }
            }
        }
    }

    Ok(processed_files)
}

pub fn copy_mod_files_to_game_data_dir(
    mod_path: &Path,
    game_data_path: &Path,
) -> anyhow::Result<Vec<String>> {
    // 根据目录下的文件进行安装
    let mut processed_files = vec![];

    // 处理源目录中的所有符合格式的文件
    // entries 就是目录中的所有文件和文件夹的集合
    let entries = fs::read_dir(mod_path)?;

    for entry in entries {
        let entry = entry?;
        let record_file_path = entry.path();

        let record_file_name = record_file_path
            .file_name()
            .map(|os_str| os_str.to_string_lossy());

        if let Some(file_name) = record_file_name {
            // 如果是以xxxxxx开头的.patch_数字结尾的主补丁文件才会进行下一步处理，prefix为xxxxxx，不符合则为空字符串
            let prefix = extract_name(&file_name);
            if !prefix.is_empty() {
                // 提取游戏里该前缀名称的最大编号
                let mut max_number = get_game_data_dir_max_number(prefix, game_data_path)?;

                // 下一个数字补丁编号，顺序+1
                max_number += 1;

                // 计算出将要安装的名称和路径
                let game_mod_patch_name = format!("{}.patch_{}", prefix, max_number);
                let game_mod_patch_path = game_data_path.join(&game_mod_patch_name);

                println!(
                    "主补丁 {} 将要被安装到 {}",
                    record_file_path.display(),
                    game_mod_patch_path.display()
                );

                // 将文件复制到目标目录并重命名
                fs::copy(&record_file_path, &game_mod_patch_path)?;

                // 记录已安装的文件
                processed_files.push(game_mod_patch_name.clone());

                // 处理可能存在的辅补丁文件 `.gpu_resources` 和 `.stream`
                let extra_extensions = [".gpu_resources", ".stream"];
                for ext in &extra_extensions {
                    // 计算出可能存在的补丁名称和路径
                    let extra_file_name = format!("{}{}", file_name, ext);
                    let extra_file_path = mod_path.join(&extra_file_name);

                    // 根据路径直接判断补丁文件是否存在
                    if extra_file_path.exists() {
                        // 计算出将要安装的名称和路径
                        let game_mod_extra_name = format!("{}{}", game_mod_patch_name, ext);
                        let game_mod_extra_path = game_data_path.join(&game_mod_extra_name);

                        println!(
                            "辅补丁 {} 将要被安装到 {}",
                            extra_file_path.display(),
                            game_mod_extra_path.display()
                        );

                        fs::copy(&extra_file_path, &game_mod_extra_path)?;

                        processed_files.push(game_mod_extra_name);
                    }
                }
            }
        }
    }
    Ok(processed_files)
}

// 卸载Mod
#[tauri::command(async, rename_all = "snake_case")]
pub fn uninstall_mods(app: AppHandle, all_mod_flag: u8) -> Result<(), String> {
    // 获取本地配置的目录
    let config_json = my_config::get_app_config_json(&app, "game_mod").unwrap_or(json!({}));
    let game_data_path = config_json
        .get("game_data_path")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    if game_data_path.is_empty() {
        return Err(format!("未配置游戏目录"));
    }

    let game_data_path = Path::new(game_data_path);

    if all_mod_flag == 0 {
        println!("删除本软件安装的Mod");
        uninstall_db_mods(game_data_path)?;
    } else {
        println!("删除所有Mod");
        uninstall_all_mods(game_data_path)?;
    }
    Ok(())
}

// 删除所有游戏补丁文件
fn uninstall_all_mods(game_data_path: &Path) -> anyhow::Result<(), String> {
    // 清除安装记录
    my_db::uninstall_all_mods().map_err(|e| format!("Mod记录删除失败：{}", e))?;

    for entry in fs::read_dir(game_data_path).map_err(|e| format!("读取游戏目录失败：{}", e))?
    {
        let entry = entry.map_err(|e| format!("游戏目录获取失败：{}", e))?;
        let path = entry.path();

        if path.is_file() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.contains(".patch_") {
                    println!("删除文件: {}", name);
                    fs::remove_file(&path).map_err(|e| format!("游戏补丁删除失败：{}", e))?;
                }
            }
        }
    }
    Ok(())
}

// 删除通过本软件安装的所有补丁文件
fn uninstall_db_mods(game_data_path: &Path) -> anyhow::Result<(), String> {
    // 查询已安装mod
    let files = my_db::get_all_mods_install_files().map_err(|e| format!("{}", e))?;

    // 清除安装记录
    my_db::uninstall_all_mods().map_err(|e| format!("Mod记录删除失败：{}", e))?;

    // 根据查询到的文件files进行遍历删除
    let errors: Vec<String> = files
        .into_par_iter()
        .map(|file| {
            let file_path: PathBuf = game_data_path.join(&file);

            if file_path.exists() {
                fs::remove_file(&file_path)
                    .map_err(|e| format!("删除mod文件 {:?} 失败: {}", file_path, e))
            } else {
                Ok(())
            }
        })
        .filter_map(Result::err) // 过滤出错误
        .collect();

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join("\n")) // 返回所有错误信息
    }
}

fn get_mod_manifest_json(path: &Path) -> anyhow::Result<Option<Manifest>> {
    let manifest_json_path = path.join("manifest.json");
    if manifest_json_path.exists() && manifest_json_path.is_file() {
        println!("找到文件: {:?}", manifest_json_path);
        // 读取文件内容
        let content = fs::read_to_string(&manifest_json_path)?;
        // 解析为结构体
        let config = json5::from_str(&content)?;
        Ok(Some(config))
    } else {
        println!(" {:?}目录中不存在manifest.json", manifest_json_path);
        Ok(None)
    }
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_env_mod_info(id: u64, env_id: u32) -> Result<GameMod, String> {
    let mods = my_db::get_env_mod_info(id, env_id).map_err(|e| format!("查询失败: {}", e))?;
    Ok(mods)
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_env_mod_info_with_install_files(
    record_id: u64,
    env_id: u32,
) -> Result<GameModData, String> {
    let mod_info = my_db::get_env_mod_info(record_id, env_id)
        .map_err(|e| format!("查询Mod信息失败: {}", e))?;

    let files = my_db::get_env_mod_install_files(env_id, record_id)
        .map_err(|e| format!("查询安装文件失败: {}", e))?;

    let mod_data = GameModData {
        info: mod_info,
        files: files,
    };

    Ok(mod_data)
}

#[tauri::command(rename_all = "snake_case")]
pub fn up_mod_info(app: AppHandle, mut mod_info: GameMod, up_img_flag: bool) -> Result<(), String> {
    // 获取本地配置的目录
    let config_json = my_config::get_app_config_json(&app, "game_mod").unwrap_or(json!({}));
    let mods_store_path = config_json
        .get("mods_store_path")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    if mods_store_path.is_empty() {
        return Err(format!("存档目录未设置"));
    }

    let record_path = Path::new(mods_store_path);

    let target_dir = record_path.join(&mod_info.path);

    // 检查mod文件夹是否存在
    if !target_dir.is_dir() {
        return Err(format!("Mod目录：{} 不存在", target_dir.display()));
    }

    // 添加预览图片，如果图片没有改变则无需添加
    if up_img_flag {
        mod_info.icon = my_fs::copy_preview_img(&mod_info.icon, &target_dir)?;
    } else {
        mod_info.icon = Path::new(&mod_info.icon)
            .file_name() // 获取路径最后一部分
            .map(|s| s.to_string_lossy().into_owned()) // 转换为 String
            .unwrap_or_else(|| "".to_string());

        println!("mod_info.icon: {}", &mod_info.icon)
    }

    // 更新
    my_db::update_mod_record_info(&mod_info).map_err(|e| format!("更新失败: {}", e))?;
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub fn delete_one_mod(
    app: AppHandle,
    record_id: u64,
    env_id: u32,
    delete_file_flag: u8,
    mod_dir_name: &str,
) -> Result<(), String> {
    // 获取本地配置的目录
    let config_json = my_config::get_app_config_json(&app, "game_mod").unwrap_or(json!({}));
    let mods_store_path = config_json
        .get("mods_store_path")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    if mods_store_path.is_empty() {
        return Err(format!("获取Mod存档目录失败"));
    }

    let mods_store_path = Path::new(mods_store_path);

    // 先通过mods_install_files表查询该mod有没有正在使用，没有则允许删除记录/文件
    let mod_install_status = my_db::check_mod_install_status(record_id)
        .map_err(|e| format!("查询Mod安装状态失败: {}", e))?;
    if mod_install_status {
        return Err(format!("该Mod正在游戏内使用，请先清除！"));
    }
    my_db::delete_one_mod(record_id, env_id, delete_file_flag)
        .map_err(|e| format!("删除Mod记录失败: {}", e))?;

    if delete_file_flag == 1 {
        let mod_path = mods_store_path.join(mod_dir_name);
        if mod_path.exists() {
            fs::remove_dir_all(&mod_path)
                .map_err(|e| format!("删除Mod {:?} 失败，请手动删除！错误原因: {}", mod_path, e))?;
        }
    }
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub fn update_environment_mod_sort(
    env_mod_id: u64,
    pre_sort: &str,
    next_sort: &str,
) -> Result<String, String> {
    let env_mod_sort = get_lexorank_between_value(pre_sort, next_sort)
        .map_err(|e| format!("获取排序值失败: {}", e))?;
    my_db::update_environment_mod_sort(env_mod_id, &env_mod_sort)?;
    Ok(env_mod_sort)
}

#[tauri::command(rename_all = "snake_case")]
pub fn add_environment(name: &str) -> Result<(), String> {
    my_db::add_environment(name).map_err(|e| format!("添加失败: {}", e))?;
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub fn delete_environment(id: u32) -> Result<(), String> {
    let num = my_db::get_environment_install_mods_num(id)
        .map_err(|e| format!("删除失败-检查环境Mod安装状态失败: {}", e))?;
    if num > 0 {
        return Err(format!("本环境有Mod正在游戏里使用，请先卸载！"));
    }
    my_db::delete_environment(id).map_err(|e| format!("删除失败: {}", e))?;
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub fn copy_environment(id: u32, name: &str) -> Result<(), String> {
    my_db::copy_environment(id, name).map_err(|e| format!("复制失败: {}", e))?;
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub fn add_mods_to_environment(env_id: u32, ids: Vec<u64>) -> Result<(), String> {
    if ids.len() == 0 {
        return Err(format!("请至少选择一个Mod进行添加！"));
    }

    let mods = my_db::get_mods_records_by_ids(ids).map_err(|e| format!("查找Mod失败: {}", e))?;
    println!("add_mods_to_environment get mods: {:#?}", mods);

    let sorts = get_lexorank_next_value(env_id, mods.len())
        .map_err(|e| format!("生成排序值失败: {}", e))?;

    my_db::add_mods_to_environment(env_id, mods, sorts).map_err(|e| format!("添加失败: {}", e))?;

    Ok(())
}

fn get_lexorank_next_value(env_id: u32, n: usize) -> anyhow::Result<Vec<String>> {
    let table = SymbolTable::base36();
    let env_mod_last_sort = my_db::get_environment_mod_last_sort(env_id)?;
    let sorts = table
        .mudder(n, env_mod_last_sort.as_deref(), None)
        .map_err(|e| anyhow::anyhow!(e))?;

    Ok(sorts)
}

fn get_lexorank_between_value(pre_sort: &str, next_sort: &str) -> anyhow::Result<String> {
    let table = SymbolTable::base36();
    let mut start: Option<&str> = None;
    let mut end: Option<&str> = None;

    if !pre_sort.is_empty() {
        start = Some(pre_sort);
    }

    if !next_sort.is_empty() {
        end = Some(next_sort);
    }

    let sort = table
        .mudder_one(start, end)
        .map_err(|e| anyhow::anyhow!(e))?;

    Ok(sort)
}
