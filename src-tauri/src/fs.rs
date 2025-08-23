use anyhow::{Context, Result};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use unrar::Archive;
use zip::ZipArchive;

// 使用系统文件管理器打开文件夹
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

// 目录复制
pub fn dir_copy(src: &Path, dst: &Path) -> Result<()> {
    if src != dst {
        // 使用 create_dir_all 创建所有需要的嵌套目录
        fs::create_dir_all(dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let dest_path = dst.join(entry.file_name());
            if file_type.is_dir() {
                dir_copy(&entry.path(), &dest_path)?;
            } else {
                fs::copy(entry.path(), dest_path)?;
            }
        }
    }

    Ok(())
}

// 目录移动
#[allow(dead_code)]
pub fn dir_move(src: &Path, dst: &Path) -> Result<()> {
    dir_copy(src, dst)?;
    fs::remove_dir_all(src)?;

    Ok(())
}

// 是否只有一个目录
pub fn has_only_one_directory(path: &Path) -> Result<bool> {
    let entries = fs::read_dir(path)?
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    if entries.len() != 1 {
        return Ok(false);
    }

    Ok(entries[0].file_type()?.is_dir())
}

// 移动第一个目录下的所有文件到父目录
pub fn move_first_subdir_contents_to_parent(parent_dir: &Path) -> Result<()> {
    // 读取目录项，跳过文件，只找第一个子目录
    let first_subdir = fs::read_dir(parent_dir)?
        .filter_map(Result::ok)
        .find(|entry| entry.path().is_dir())
        .context("没有找到子目录")?;

    let subdir_path = first_subdir.path();

    // 遍历子目录中的内容并移动到父目录
    for entry in fs::read_dir(&subdir_path)? {
        let entry = entry?;
        let from = entry.path();
        let file_name = entry.file_name(); // 获取文件名（OsStr）
        let to = parent_dir.join(&file_name);

        fs::rename(&from, &to).with_context(|| format!("移动 {:?} 到 {:?} 失败", from, to))?;
    }

    // 可选：删除空子目录
    fs::remove_dir_all(&subdir_path)?;

    Ok(())
}

// 获取Mod目录第一张图片
pub fn get_mod_first_image_name(path: &Path) -> String {
    let mut first_image_name = String::new();
    if path.is_dir() {
        let images_ext = [
            "jpg", "jpeg", "png", "gif", "bmp", "webp", "tiff", "ico", "svg", "jfif",
        ];

        let entries = match fs::read_dir(&path) {
            Ok(dir) => dir,
            Err(e) => {
                eprintln!("无法打开目录: {}", e);
                return first_image_name;
            }
        };

        for entry in entries.flatten() {
            let file_path = entry.path();
            let file_name = entry.file_name();
            // 获取第一张图片
            if let Some(ext) = file_path.extension().and_then(|s| s.to_str()) {
                if images_ext.contains(&ext.to_lowercase().as_str()) {
                    first_image_name = file_name.to_string_lossy().into_owned();
                    break;
                }
            }
        }
    }

    first_image_name
}

// preview_file 这里传进来的预览图是一个完整的图片地址
pub fn copy_preview_img(preview_file: &str, target_path: &Path) -> Result<String, String> {
    // 预览图片和要copy的图片是同一个路径，copy时会提示文件占用；
    // 解决方案：如果要copy的文件已经在存档目录里了，则无需copy，直接返回图片名；
    if !preview_file.is_empty() {
        let preview_path = Path::new(preview_file);

        if preview_path.is_file() {
            if let Some(extension) = preview_path.extension() {
                if let Some(ext_str) = extension.to_str() {
                    // 获取图片后缀名生成新名字 和 目标路径
                    let preview_img = format!("preview.{}", ext_str);
                    let target_preview_path = target_path.join(&preview_img);

                    // 目标路径与源文件路径一致的话就跳过copy操作
                    if target_preview_path != preview_path {
                        fs::copy(&preview_path, &target_preview_path).map_err(|e| {
                            format!(
                                "预览图复制失败 '{}' to '{}': {}",
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

// https://github.com/zip-rs/zip2/blob/master/examples/extract.rs
pub fn unzip_one_file(zip_path: &str, dest_path: &str) -> Result<()> {
    let file = fs::File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => PathBuf::from(dest_path).join(path),
            None => continue,
        };

        // 如果 ZIP 文件中的某个文件附加了注释，就打印出来。
        // {
        //     let comment = file.comment();
        //     if !comment.is_empty() {
        //         println!("File {i} comment: {comment}");
        //     }
        // }

        if file.is_dir() {
            // println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath)?;
        } else {
            // println!(
            //     "File {} extracted to \"{}\" ({} bytes)",
            //     i,
            //     outpath.display(),
            //     file.size()
            // );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)?;
                }
            }
            let mut outfile = fs::File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }

        // Get and Set permissions
        // #[cfg(unix)]
        // {
        //     use std::os::unix::fs::PermissionsExt;

        //     if let Some(mode) = file.unix_mode() {
        //         fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
        //     }
        // }
    }

    Ok(())
}

// https://github.com/muja/unrar.rs/blob/master/examples/basic_extract.rs
pub fn unrar_one_file(zip_path: &str, dest_path: &str) -> Result<()> {
    let mut archive = Archive::new(zip_path).open_for_processing()?;
    while let Some(header) = archive.read_header()? {
        // println!(
        //     "{} bytes: {}",
        //     header.entry().unpacked_size,
        //     header.entry().filename.to_string_lossy(),
        // );
        archive = if header.entry().is_file() {
            header.extract_with_base(dest_path)?
        } else {
            header.skip()?
        };
    }
    Ok(())
}

// https://github.com/hasenbanck/sevenz-rust2/blob/main/examples/decompress.rs
pub fn un7z_one_file(zip_path: &str, dest_path: &str) -> Result<()> {
    sevenz_rust2::decompress_file(zip_path, dest_path)?;
    Ok(())
}
