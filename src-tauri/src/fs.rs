use crate::db::{add_mod, GameMod, GameModData};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;

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

#[tauri::command]
pub fn get_folder_first_image(folder_path: String) -> Result<String, String> {
    let path = PathBuf::from(&folder_path);

    if !path.is_dir() {
        return Err(format!("{} 不是一个有效的文件夹路径", folder_path));
    }

    let images_ext = ["jpg", "jpeg", "png", "gif", "bmp", "webp"]; // 支持的图片格式

    let entries = fs::read_dir(&path).map_err(|e| format!("无法读取文件夹: {}", e))?;

    for entry in entries.flatten() {
        let file_path = entry.path();

        if let Some(ext) = file_path.extension().and_then(|s| s.to_str()) {
            if images_ext.contains(&ext.to_lowercase().as_str()) {
                return Ok(file_path.to_string_lossy().to_string());
            }
        }
    }

    Err("未找到任何图片文件".to_string())
}

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

#[tauri::command(rename_all = "snake_case")]
pub fn copy_files(source_dir: &str, target_dir: &str, preview_file: &str) -> Result<String, String> {
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
    if !preview_file.is_empty() {
        let preview_path = Path::new(preview_file);
        if preview_path.is_file() {
            if let Some(extension) = preview_path.extension() {
                if let Some(ext_str) = extension.to_str() {
                    let ext = format!("preview.{}", ext_str);
                    let target_preview_path = target_path.join(&ext);
                    fs::copy(&preview_path, &target_preview_path).map_err(|e| {
                        format!(
                            "Failed to copy file '{}' to '{}': {}",
                            preview_path.display(),
                            target_preview_path.display(),
                            e
                        )
                    })?;

                    return Ok(ext);
                }
            }
        }
    }
    Ok(String::from(""))
}

/// 将源目录中的所有符合格式的文件批量传入目标目录，并按规则重命名
///
/// # 参数
/// - `down_dir`: 源目录路径
/// - `data_dir`: 目标目录路径
/// - `prefix`: 文件前缀（例如 `9ba626afa44a3aa3`）
///
/// # 返回
/// - `Result<Vec<String>, String>`: 返回成功处理的文件列表，或错误信息
#[tauri::command(rename_all = "snake_case")]
pub fn copy_and_rename_files(
    down_dir: &str,
    record_dir: &str,
    data_dir: &str,
    mut mod_info: GameMod,
) -> Result<(), String> {
    // 把mod存档
    let preview_file = &mod_info.preview;
    mod_info.preview = copy_files(down_dir, record_dir, preview_file)?;
    let prefix = String::from("9ba626afa44a3aa3");

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

        // 提取 `data` 目录中的最大编号
        let mut max_number = -1;
        let mut no_start = -1;

        let patch_prefix = format!("{}.patch_", prefix);

        for entry in fs::read_dir(data_path)
            .map_err(|e| format!("Failed to read target directory: {}", e))?
        {
            let entry = entry.map_err(|e| format!("Failed to process directory entry: {}", e))?;
            let tmp_var = entry.file_name();
            let file_name = tmp_var.to_string_lossy();

            if let Some(number_str) = file_name.strip_prefix(&patch_prefix) {
                if let Ok(number) = number_str.parse::<i32>() {
                    // println!("data目录-排序数字: {}", number);
                    max_number = max_number.max(number);
                    no_start = max_number + 1;
                } else {
                    // println!("data目录-不解析排序: {}", number_str);
                }
            }
        }

        println!("最大排序数字: {}", max_number);

        // 处理源目录中的所有符合格式的文件
        let src_dir = fs::read_dir(src_path).map_err(|e| format!("Failed to read source directory: {}", e))?;
        for entry in src_dir{
            let entry = entry.map_err(|e| format!("Failed to process directory entry: {}", e))?;
            let src_file_path = entry.path();

            // 仅处理以 `prefix.patch_` 开头的文件
            if let Some(file_name) = src_file_path
                .file_name()
                .map(|os_str| os_str.to_string_lossy())
            {
                if file_name.starts_with(&patch_prefix)
                    && !file_name.contains(".gpu_resources")
                    && !file_name.contains(".stream")
                {
                    // 计算下一个编号
                    max_number += 1;
                    let new_patch_name = format!("{}.patch_{}", prefix, max_number);
                    let new_patch_path = data_path.join(&new_patch_name);
                    println!(
                        "源路径：{}，新名称及路径：{}",
                        src_file_path.display(),
                        new_patch_path.display()
                    );

                    // 将文件复制到目标目录并重命名
                    fs::copy(&src_file_path, &new_patch_path).map_err(|e| {
                        format!(
                            "Failed to copy file '{}' to '{}': {}",
                            src_file_path.display(),
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
    }

    println!("mod_data: {:#?}", mod_info);

    // let no_array = vec![];
    let game_mod = GameModData {
        info: mod_info,
        // no_array: no_array,
        files: processed_files,
    };

    println!("game_mod: {:#?}", game_mod);
    add_mod(game_mod).map_err(|e| format!("已存档/安装，但记录失败，请手动添加记录或重新安装。错误原因: {}", e))?;

    Ok(())
}

fn _empty_to_none(s: Option<String>) -> Option<String> {
    s.filter(|string| !string.is_empty())
}
