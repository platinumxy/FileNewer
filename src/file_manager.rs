use std::{env, io};
use std::env::VarError;
use std::ffi::{OsStr, OsString};
use std::path::Path;
use std::time::SystemTime;
use std::fs;
use std::fs::{DirEntry, File};
use std::io::BufRead;

pub struct FileInfo {
    pub(crate) is_dir: bool,
    pub(crate) can_be_written: bool,
    pub(crate) is_link: bool,  // Adds SymLink support, though not many will have this
    pub(crate) file_name: OsString,
    pub(crate) file_ext: Option<String>,
    pub(crate) file_size: u64,
    pub(crate) last_access: Option<SystemTime>,
    pub(crate) last_modification: Option<SystemTime>,
    pub(crate) creation_time: Option<SystemTime>
}

pub fn evaluate_path_vars(user_facing_path: &str) -> Result<String, VarError> {
    let win_env_path:fn(&str) -> Result<String, VarError> = |path: &str| {
        let mut path_str = env::var(path)?;
        path_str.push_str("\\");
        Ok(path_str)
    };
    let mut new_path = user_facing_path.replace("/", "\\");
    if new_path.starts_with("~")  { new_path = new_path.replacen("~","%USERPROFILE%", 1) }
    if new_path.starts_with("\\") { new_path = new_path.replacen("\\","%SYSTEMDRIVE%",1) }

    let mut final_path = if new_path.starts_with('%') {
        let parts: Vec<&str> = new_path.split('%').collect();
        if parts.len() < 2 {
            String::from(new_path)
        } else {
            let mut path = win_env_path(parts[1])?;
            for part in &parts[2..] {
                path.push_str(part);
            }
            path
        }
    } else {
        String::from(new_path)
    };

    // Ensure the path ends with a /
    if !final_path.ends_with("\\") {
        final_path.push_str("\\");
    }

    Ok(final_path.to_string().replace("\\\\", "\\"))
}

pub fn check_dir_exists(path: &String) -> bool{
    Path::is_dir(path.as_ref())
}

pub fn get_files_in_dir<P: AsRef<Path>>(path: &P) -> io::Result<Vec<FileInfo>> {
    fs::read_dir(path)?.into_iter()
        .filter_map(|entry| {
            let dir = entry.ok()?;
            let meta = dir.metadata().ok()?;

            let file_ext = dir.path().extension()
                .and_then(OsStr::to_str)
                .map(|s| s.to_owned());

            Some(Ok(FileInfo {
                is_dir: meta.is_dir(),
                can_be_written: !meta.permissions().readonly(),
                is_link: meta.file_type().is_symlink(),
                file_name: dir.file_name(),
                file_ext,
                file_size: meta.len(),
                last_access: meta.accessed().ok(),
                last_modification: meta.modified().ok(),
                creation_time: meta.created().ok()
            }))
        }).collect()
}

pub fn get_file_ext(path: String) -> Option<String>{
    let parts: Vec<&str> = path.split('.').collect();
    if parts.len() != 1 {
        Option::from(parts.last().unwrap().to_string())
    } else {
        None
    }
}

pub fn get_file_info<P: AsRef<Path>>(path: &P) -> io::Result<FileInfo>{
    let path = path.as_ref();
    let meta = fs::metadata(&path)?;

    let file_ext = path.extension()
        .and_then(OsStr::to_str)
        .map(|s| s.to_owned());

    let file_name = path.file_name()
        .map(|os_str| os_str.to_os_string())
        .unwrap_or_else(|| OsString::from(""));

    Ok(FileInfo {
        is_dir: meta.is_dir(),
        can_be_written: !meta.permissions().readonly(),
        is_link: meta.file_type().is_symlink(),
        file_name: OsString::from(file_name),
        file_ext,
        file_size: meta.len(),
        last_access: meta.accessed().ok(),
        last_modification: meta.modified().ok(),
        creation_time: meta.created().ok()
    })
}