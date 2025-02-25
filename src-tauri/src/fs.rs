use crate::db;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;

/// 用于打开一个文件夹选择对话框，并阻塞当前线程，直到用户选择一个文件夹或取消选择。
#[allow(dead_code)]
#[tauri::command]
pub fn select_one_folder(app: AppHandle) {
    if let Some(folder_path) = app.dialog().file().blocking_pick_folder() {
        // 用户选择了文件夹
        println!("用户选择的文件夹路径: {:?}", folder_path);
    } else {
        // 用户取消了选择
        println!("没有选择文件夹");
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModInfo {
    first_image_path: String,
    mod_type: String,
    files: Vec<String>,
}

/// 获取指定mod目录的下的文件列表、第一个图片地址、自动判断mod类型
#[tauri::command]
pub fn get_mod_info(folder_path: String) -> Result<ModInfo, String> {
    let path = PathBuf::from(&folder_path);

    if !path.is_dir() {
        return Err(format!("{} 不是一个有效的文件夹路径", folder_path));
    }

    let mut mod_info = ModInfo {
        first_image_path: String::from(""),
        mod_type: String::from("voice"),
        files: vec![],
    };

    let images_ext = ["jpg", "jpeg", "png", "gif", "bmp", "webp", "tiff", "ico", "svg", "jfif"];

    let entries = fs::read_dir(&path).map_err(|e| format!("无法读取文件夹: {}", e))?;

    for entry in entries.flatten() {
        let file_path = entry.path();

        let file_path_string = file_path.to_string_lossy().into_owned();

        // 判断类型
        if file_path_string.contains("9ba626afa44a3aa3") {
            mod_info.mod_type = "model".to_string();
        }

        // 目录下有哪些文件
        mod_info.files.push(
            file_path
                .file_name()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_else(|| "unknown".to_string()),
        );

        // 获取第一张图片
        if let Some(ext) = file_path.extension().and_then(|s| s.to_str()) {
            if images_ext.contains(&ext.to_lowercase().as_str())
                && mod_info.first_image_path.is_empty()
            {
                mod_info.first_image_path = file_path.to_string_lossy().into_owned();
            }
        }
    }

    Ok(mod_info)
}

#[tauri::command(rename_all = "snake_case")]
pub fn up_mod_info(
    mut mod_info: db::GameMod,
    record_dir: &str,
    up_img_flag: bool,
) -> Result<(), String> {
    let record_path = Path::new(record_dir);

    // 检查存档文件夹是否存在
    if !record_path.is_dir() {
        return Err(format!("mod目录 '{}' 不存在", record_dir));
    }

    let target_dir = record_path.join(&mod_info.name);

    // 检查mod文件夹是否存在
    if !target_dir.is_dir() {
        return Err(format!("mod目录 '{}' 不存在", target_dir.display()));
    }

    let preview_file = &mod_info.preview;

    // 添加预览图片
    if up_img_flag {
        mod_info.preview = copy_preview_img(preview_file, &target_dir)?;
    } else {
        mod_info.preview = Path::new(preview_file)
            .file_name() // 获取路径最后一部分
            .map(|s| s.to_string_lossy().into_owned()) // 转换为 String
            .unwrap_or_else(|| "".to_string());
    }

    // 更新
    db::update_mod_record_info(&mod_info).map_err(|e| format!("更新失败: {}", e))?;
    Ok(())
}

/// 使用系统文件管理器打开文件夹
#[tauri::command]
pub fn open_folder(path: String) -> Result<(), String> {
    if !Path::new(&path).exists() {
        return Err("路径不存在".to_string());
    }
    let mut command = if cfg!(target_os = "windows") {
        Command::new("explorer")
    } else if cfg!(target_os = "macos") {
        Command::new("open")
    } else if cfg!(target_os = "linux") {
        Command::new("xdg-open")
    } else {
        return Err("不支持的操作系统".to_string());
    };

    command
        .arg(&path)
        .spawn()
        .map_err(|e| format!("打开文件夹失败: {}", e))?;

    Ok(())
}

/// mod存档：从下载目录copy到存档目录
#[tauri::command(rename_all = "snake_case")]
pub fn copy_files_to_record(
    source_dir: &str,
    target_dir: &str,
    preview_file: &str,
) -> Result<String, String> {
    let source_path = Path::new(source_dir);
    let target_path = Path::new(target_dir);

    // 检查源文件夹是否存在
    if !source_path.is_dir() {
        return Err(format!("Source directory '{}' does not exist.", source_dir));
    }

    // 创建目标文件夹（如果不存在）
    if !target_path.exists() {
        fs::create_dir_all(target_path)
            .map_err(|e| format!("Failed to create target directory: {}", e))?;
    }

    // 遍历源文件夹内的所有文件进行copy
    for entry in
        fs::read_dir(source_path).map_err(|e| format!("Failed to read source directory: {}", e))?
    {
        let entry = entry.map_err(|e| format!("Failed to process entry: {}", e))?;
        let source_file_path = entry.path();

        // 检查是否是文件（忽略子目录）
        if source_file_path.is_file() {
            // 构造目标文件路径
            let file_name = source_file_path
                .file_name()
                .ok_or("Failed to get file name.")?;
            let target_file_path = target_path.join(file_name);

            // 复制文件
            fs::copy(&source_file_path, &target_file_path).map_err(|e| {
                format!(
                    "Failed to copy file '{}' to '{}': {}",
                    source_file_path.display(),
                    target_file_path.display(),
                    e
                )
            })?;
        }
    }

    // 在mod文件夹里放入预览图片并改名为preview
    let preview_img = copy_preview_img(preview_file, target_path)?;

    Ok(preview_img)
}

fn copy_preview_img(preview_file: &str, target_path: &Path) -> Result<String, String> {
    // 预览图片和要copy的图片是同一个路径，copy时会提示文件占用；
    // 解决方案：如果要copy的文件已经在存档目录里了，则无需copy，直接返回图片名；
    if !preview_file.is_empty() {
        let preview_path = Path::new(preview_file);

        // let is_same_directory = match preview_path.parent() {
        //     Some(parent) => parent == target_dir,
        //     None => false,
        // };

        if preview_path.is_file() {
            if let Some(extension) = preview_path.extension() {
                if let Some(ext_str) = extension.to_str() {
                    let preview_img = format!("preview.{}", ext_str);
                    let target_preview_path = target_path.join(&preview_img);

                    // 目标路径与文件路径一致的话就跳过copy操作
                    if target_preview_path != preview_path {
                        fs::copy(&preview_path, &target_preview_path).map_err(|e| {
                            format!(
                                "Failed to copy file '{}' to '{}': {}",
                                preview_path.display(),
                                target_preview_path.display(),
                                e
                            )
                        })?;
                    }

                    return Ok(preview_img);
                }
            }
        }
    }
    Ok(String::from(""))
}

/// 从下载目录选择mod进行存档；如果安装置为是，则额外进行一步操作：从下载目录安装到游戏data目录
/// # 参数
/// - `down_dir`: 下载目录
/// - `record_dir` 存档目录
/// - `data_dir`: 游戏data目录
/// - `mod_info`: mod信息
///
/// # 返回
/// - `Result<(), String>`: 成功返回空，失败返回错误信息
#[tauri::command(rename_all = "snake_case")]
pub fn down_copy_and_rename_files(
    down_dir: &str,
    record_dir: &str,
    data_dir: &str,
    mut mod_info: db::GameMod,
) -> Result<(), String> {
    // 从下载目录把mod存档并返回预览图片
    let preview_file = &mod_info.preview;
    mod_info.preview = copy_files_to_record(down_dir, record_dir, preview_file)?;

    let mut processed_files = vec![];
    if mod_info.activate {
        let src_path = Path::new(down_dir);
        let data_path = Path::new(data_dir);

        // 检查源目录和目标目录是否存在
        if !src_path.is_dir() {
            return Err(format!("Source directory '{}' does not exist.", down_dir));
        }
        if !data_path.is_dir() {
            return Err(format!("Target directory '{}' does not exist.", data_dir));
        }

        if mod_info.mod_type == "model" {
            processed_files = copy_and_rename_mod_files(src_path, data_path)?;
        } else {
            processed_files = copy_and_rename_voice_files(src_path, data_path)?;
        }
    }

    // println!("mod_data: {:#?}", mod_info);

    let game_mod = db::GameModData {
        info: mod_info,
        files: processed_files,
    };

    // println!("game_mod: {:#?}", game_mod);
    db::add_mod(game_mod).map_err(|e| format!("已添加，但生成添加记录失败。错误原因: {}", e))?;

    Ok(())
}

/// 根据id删除某个存档，前提是已禁用
#[tauri::command(rename_all = "snake_case")]
pub fn remove_dir_all(dir_path: &str, record_id: i64) -> Result<(), String> {
    // 不存在直接删除数据库里的记录；存在的话先查询安装状态，没有安装的才能删除
    if Path::new(dir_path).exists() {
        db::check_mod_activate(record_id)?;
        if fs::remove_dir_all(dir_path).is_ok() {
            db::del_one_mod_record(record_id).map_err(|e| format!("删除mod存档失败: {}", e))?;

            println!("该目录 {} 删除成功", dir_path);
        } else {
            println!("该目录 {} 不存在或删除失败", dir_path);
            return Err("删除失败".to_string());
        }
    } else {
        db::del_one_mod_record(record_id).map_err(|e| format!("删除mod存档失败: {}", e))?;
    }
    println!("该目录 {} 下的文件已删除", dir_path);
    Ok(())
}

/// 一键卸载所有mod
#[tauri::command(rename_all = "snake_case")]
pub fn uninstall_mods_all(data_dir: &str, mod_type: &str) -> Result<(), String> {
    let dir_path = Path::new(data_dir);
    if !dir_path.exists() {
        return Err(format!("目录'{}'不存在", data_dir));
    }

    // 先从数据库mods_install_files表查询已安装的文件记录
    let files = db::get_all_mods_install_files(mod_type).map_err(|e| format!("{}", e))?;

    // 再从数据库mods_install_files表删除已安装的文件记录
    db::uninstall_all_mod(mod_type).map_err(|e| format!("mod卸载失败：{}", e))?;

    // 根据查询到的文件进行遍历删除
    let errors: Vec<String> = files
        .into_par_iter()
        .map(|file| {
            let file_path: PathBuf = dir_path.join(&file);

            if file_path.exists() {
                fs::remove_file(&file_path).map_err(|e| format!("删除 {:?} 失败: {}", file_path, e))
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

#[derive(Serialize, Deserialize, Debug)]
pub struct ModsActivate {
    pub id: i64,
    pub activate: bool,
}

/// 应用安装：根据列表中的安装状态进行安装；
/// 步骤：先一键卸载，再根据数据库里的状态值和前端传过来的部分状态值进行安装（要做数据融合，前端传过来的覆盖数据库里的值）
/// 前端可能只传过来一个mod的状态值，其它的默认取数据库里的值
#[tauri::command(rename_all = "snake_case")]
pub fn deploy_mods(
    record_dir: &str,
    data_dir: &str,
    mode_type: &str,
    mut mod_activate_data: Vec<ModsActivate>,
) -> Result<(), String> {
    let mut mods = db::get_mod_records(mode_type, "")?;
    // println!("old mods: {:#?}", mods);

    for item in &mut mods {
        // 查找id相同的元素，并更新activate值
        if let Some(element) = mod_activate_data
            .iter_mut()
            .find(|kk| kk.id == item.id.unwrap_or(0))
        {
            item.activate = element.activate;
        }
    }

    println!("record_dir: {}", record_dir);
    println!("data_dir: {}", data_dir);
    println!("new mods: {:#?}", mods);

    // 先一键卸载
    uninstall_mods_all(data_dir, mode_type).map_err(|e| format!("mod部署时卸载失败：{}", e))?;

    // 根据activate安装mod，从存档目录进行安装；更新mods_records安装状态和添加mods_install_files记录
    let mut errors = Vec::new(); // 用于收集错误信息

    for item in mods {
        let name: String = item.name.clone();
        if let Err(e) = record_copy_and_rename_files(record_dir, data_dir, item) {
            errors.push(format!("处理 {} 失败: {}", name, e));
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join("\n")) // 将所有错误合并为一个字符串
    }
}

/// 获取游戏data目录最大的number补丁值
fn get_data_dir_max_number(data_path: &Path, prefix: &str) -> Result<i32, String> {
    // 提取 `data` 目录中的最大编号
    let mut max_number = -1;
    let patch_prefix = format!("{}.patch_", prefix);

    for entry in
        fs::read_dir(data_path).map_err(|e| format!("Failed to read target directory: {}", e))?
    {
        let entry = entry.map_err(|e| format!("Failed to process directory entry: {}", e))?;
        let tmp_var = entry.file_name();
        let file_name = tmp_var.to_string_lossy();

        // strip_prefix去掉匹配的前缀；number_str.parse::<i32>()尝试解析字符串为i32
        if let Some(number_str) = file_name.strip_prefix(&patch_prefix) {
            if let Ok(number) = number_str.parse::<i32>() {
                // println!("data目录-排序数字: {}", number);
                max_number = max_number.max(number);
            } else {
                // 如果不是字符串形式的i32整数会解析失败，意味着无法解析字符串和数字的组合
                // println!("data目录-解析失败: {}", number_str);
            }
        }
    }

    println!("最大排序数字: {}", max_number);
    Ok(max_number)
}

/// 单个模型mod目录里的的多个补丁文件的复制、重命名函数
fn copy_and_rename_mod_files(src_path: &Path, data_path: &Path) -> Result<Vec<String>, String> {
    let prefix = String::from("9ba626afa44a3aa3");
    let mut max_number = get_data_dir_max_number(data_path, &prefix)?;
    let patch_prefix = format!("{}.patch_", prefix);

    // 处理源目录中的所有符合格式的文件
    // entries 就是目录中的所有文件和文件夹的集合
    let entries = fs::read_dir(src_path).map_err(|e| format!("读取存档目录失败: {}", e))?;
    let mut processed_files = vec![];

    for entry in entries {
        let entry = entry.map_err(|e| format!(": {}", e))?;
        let record_file_path = entry.path();

        if let Some(file_name) = record_file_path
            .file_name()
            .map(|os_str| os_str.to_string_lossy())
        {
            // 仅处理以 '9ba626afa44a3aa3.patch_' 开头的文件
            if file_name.starts_with(&patch_prefix)
                && !file_name.contains(".gpu_resources")
                && !file_name.contains(".stream")
            {
                // 计算新的数字编号
                max_number += 1;
                let new_patch_name = format!("{}.patch_{}", prefix, max_number);
                let new_patch_path = data_path.join(&new_patch_name);
                println!(
                    "源路径：{}，新名称及路径：{}",
                    record_file_path.display(),
                    new_patch_path.display()
                );

                // 将文件复制到目标目录并重命名
                fs::copy(&record_file_path, &new_patch_path).map_err(|e| {
                    format!(
                        "Failed to copy file '{}' to '{}': {}",
                        record_file_path.display(),
                        new_patch_path.display(),
                        e
                    )
                })?;

                processed_files.push(new_patch_name.clone());

                // 处理可能存在的附加文件 `.gpu_resources` 和 `.stream`
                let extra_extensions = [".gpu_resources", ".stream"];
                for ext in &extra_extensions {
                    let extra_file_name = format!("{}{}", file_name, ext);
                    let extra_file_path = src_path.join(&extra_file_name);
                    if extra_file_path.exists() {
                        let new_extra_name = format!("{}{}", new_patch_name, ext);
                        let new_extra_path = data_path.join(&new_extra_name);
                        println!(
                            "ext源路径：{}，新名称及路径：{}",
                            extra_file_path.display(),
                            new_extra_path.display()
                        );

                        fs::copy(&extra_file_path, &new_extra_path).map_err(|e| {
                            format!(
                                "Failed to copy file '{}' to '{}': {}",
                                extra_file_path.display(),
                                new_extra_path.display(),
                                e
                            )
                        })?;
                        processed_files.push(new_extra_name);
                    }
                }
            }
        }
    }
    Ok(processed_files)
}

/// 单个音频mod目录里的的多个补丁文件的复制、重命名函数
fn copy_and_rename_voice_files(src_path: &Path, data_path: &Path) -> Result<Vec<String>, String> {
    let mut processed_files = vec![];

    // 处理源目录中的所有符合格式的文件
    // entries 就是目录中的所有文件和文件夹的集合
    let entries = fs::read_dir(src_path).map_err(|e| format!("读取存档目录失败: {}", e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!(": {}", e))?;
        let record_file_path = entry.path();

        if let Some(file_name) = record_file_path
            .file_name()
            .map(|os_str| os_str.to_string_lossy())
        {
            let prefix = extract_name(&file_name);

            // 过滤模型文件，不处理以'9ba626afa44a3aa3'开头的文件
            // 按照extract_name函数规则提取前缀名，符合的返回名称，不符合的返回空字符串
            // 不能是图片
            if prefix != "9ba626afa44a3aa3" && !prefix.is_empty() && is_not_image(&file_name) {
                let mut max_number = get_data_dir_max_number(data_path, prefix)?;

                // 计算新的数字补丁编号
                max_number += 1;

                let new_patch_name = format!("{}.patch_{}", prefix, max_number);
                let new_patch_path = data_path.join(&new_patch_name);
                println!(
                    "源路径：{}，新名称及路径：{}",
                    record_file_path.display(),
                    new_patch_path.display()
                );

                // 将文件复制到目标目录并重命名
                fs::copy(&record_file_path, &new_patch_path).map_err(|e| {
                    format!(
                        "音频补丁主文件复制到游戏data目录失败 '{}' to '{}': {}",
                        record_file_path.display(),
                        new_patch_path.display(),
                        e
                    )
                })?;

                processed_files.push(new_patch_name.clone());

                // 处理可能存在的附加文件 `.gpu_resources` 和 `.stream`
                let extra_extensions = [".gpu_resources", ".stream"];
                for ext in &extra_extensions {
                    let extra_file_name = format!("{}{}", file_name, ext);
                    let extra_file_path = src_path.join(&extra_file_name);
                    // 如果补丁文件存在
                    if extra_file_path.exists() {
                        let new_extra_name = format!("{}{}", new_patch_name, ext);
                        let new_extra_path = data_path.join(&new_extra_name);
                        println!(
                            "ext源路径：{}，新名称及路径：{}",
                            extra_file_path.display(),
                            new_extra_path.display()
                        );

                        fs::copy(&extra_file_path, &new_extra_path).map_err(|e| {
                            format!(
                                "音频补丁扩展文件复制到游戏data目录失败 '{}' to '{}': {}",
                                extra_file_path.display(),
                                new_extra_path.display(),
                                e
                            )
                        })?;
                        processed_files.push(new_extra_name);
                    }
                }
            }
        }
    }
    Ok(processed_files)
}

/// 从Mod存档目录复制和重命名到游戏data目录
pub fn record_copy_and_rename_files(
    record_dir: &str,
    data_dir: &str,
    mod_info: db::GameMod,
) -> Result<(), String> {
    let mut processed_files = vec![];

    if mod_info.activate {
        let record_path = Path::new(record_dir);
        let data_path = Path::new(data_dir);

        let mod_name = mod_info.name.clone();
        let src_path = record_path.join(mod_name);

        // 检查源目录和目标目录是否存在
        if !src_path.is_dir() {
            return Err(format!("Source directory '{}' does not exist.", record_dir));
        }
        if !data_path.is_dir() {
            return Err(format!("Target directory '{}' does not exist.", data_dir));
        }

        if mod_info.mod_type == "model" {
            processed_files = copy_and_rename_mod_files(src_path.as_path(), data_path)?;
        } else {
            processed_files = copy_and_rename_voice_files(src_path.as_path(), data_path)?;
        }
    }
    println!(
        "{} {} processed_files: {:#?}",
        mod_info.name, mod_info.activate, processed_files
    );

    let game_mod = db::GameModData {
        info: mod_info,
        files: processed_files,
    };

    // println!("game_mod: {:#?}", game_mod);
    db::update_mod(game_mod).map_err(|e| format!("更新mod记录失败。错误原因: {}", e))?;

    Ok(())
}

/// Some空值转换成None
fn _empty_to_none(s: Option<String>) -> Option<String> {
    s.filter(|string| !string.is_empty())
}

/// 判断是否不是图片
/// fn main() {
///     let file1 = "document.txt";
///     let file2 = "photo.jpg";
///     let file3 = "no_extension";

///     println!("{}", is_not_image(file1)); // true
///     println!("{}", is_not_image(file2)); // false
///     println!("{}", is_not_image(file3)); // true
/// }
fn is_not_image(file_name: &str) -> bool {
    let image_extensions = [
        "jpg", "jpeg", "png", "gif", "bmp", "webp", "tiff", "ico", "svg", "jfif"
    ];

    match Path::new(file_name).extension() {
        Some(ext) => !image_extensions.contains(&ext.to_string_lossy().to_lowercase().as_str()),
        None => true, // 没有扩展名的文件也视为不是图片
    }
}

/// 使用示例
/// fn main() {
///     let files = vec![
///         "9ba626afa44a3aa3.patch_9",
///         "346d785d6ca13ff4.patch_0",
///         "18235e0c9ec0e636.patch_0",
///         "abcde12345.patch_0.stream",
///     ];
///     let extracted: Vec<&str> = files.iter()
///         .filter_map(|f| extract_name(f))
///         .collect();
///     println!("{:?}", extracted);
/// }
/// ["9ba626afa44a3aa3", "346d785d6ca13ff4", "18235e0c9ec0e636"]
fn extract_name(file_name: &str) -> &str {
    if let Some((name, suffix)) = file_name.split_once(".patch_") {
        if suffix.chars().all(|c| c.is_numeric()) {
            return name;
        }
    }
    ""
}

// fn extract_name_regex(file_name: &str) -> Option<String> {
//     let re = Regex::new(r"^([a-f0-9]+)\.patch_\d+$").unwrap();
//     re.captures(file_name).map(|cap| cap[1].to_string())
// }
