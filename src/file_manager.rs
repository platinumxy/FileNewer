use std::{env, io};
use std::env::VarError;
use std::ffi::{OsStr, OsString};
use std::path::Path;
use std::time::SystemTime;
use std::fs;
use std::os::windows::prelude::MetadataExt;
use chrono::{DateTime, Local};

#[derive(PartialEq)]
#[derive(Debug)]
pub enum FileType{
    WriteableDir,
    UnwritableDir,
    WritableFile,
    UnwritableFile,
    WritableLink,
    UnwritableLink,
    Unknown
}

#[derive(Debug)]
pub struct FileInfo {
    pub(crate) file_type: FileType, //
    pub(crate) can_be_written: bool,//
    pub(crate) file_name: OsString,
    pub(crate) file_ext: Option<String>,
    pub(crate) file_size: u64,
    pub(crate) last_access: Option<SystemTime>,
    pub(crate) last_modification: Option<SystemTime>,
    pub(crate) creation_time: Option<SystemTime>,
    pub(crate) is_hidden: bool,
}
impl FileInfo{
    pub fn is_dir(&self) -> bool{
        self.file_type == FileType::UnwritableDir
            || self.file_type == FileType::WriteableDir
    }

    pub fn single_char_desc(&self) -> &str{
        match self.file_type {
            FileType::UnwritableDir => { "D" }
            FileType::WriteableDir => { "d" }
            FileType::UnwritableLink => { "L" }
            FileType::WritableLink => { "l" }
            FileType::UnwritableFile => { "F" }
            FileType::WritableFile => { "f" }
            _ => { "?" }
        }
    }

    pub fn type_to_basic_str(&self) -> &str{
        match self.file_type {
            FileType::UnwritableDir => { "Directory" }
            FileType::WriteableDir => { "Directory" }
            FileType::UnwritableLink => { "Link" }
            FileType::WritableLink => { "Link" }
            FileType::UnwritableFile => { "File" }
            FileType::WritableFile => { "File" }
            _ => { "UNKNOWN" }
        }
    }

    pub fn gen_type_enum(is_dir:&bool, is_writable:&bool, is_link:&bool) -> FileType{
        match (is_dir, is_writable, is_link) {
            (true, false, false) => { FileType::UnwritableDir }
            (true, true, false) => { FileType::WriteableDir }
            (false, false, true) => { FileType::UnwritableLink }
            (false, true, true) => { FileType::WritableLink }
            (false, false, false) => { FileType::UnwritableFile }
            (false, true, false) => { FileType::WritableFile }
            _ => { FileType::Unknown }
        }
    }


    pub fn last_access_formated(&self) -> String {format_system_time_opt(self.last_access)}
    pub fn last_mod_formated(&self) -> String {format_system_time_opt(self.last_modification)}
    pub fn creation_time_formated(&self) -> String {format_system_time_opt(self.creation_time)}
}

pub fn evaluate_path_vars(user_facing_path: &str) -> Result<String, VarError> {
    let win_env_path: fn(&str) -> Result<String, VarError> = |path: &str| {
        let mut path_str = env::var(path)?;
        path_str.push_str("\\");
        Ok(path_str)
    };
    let mut new_path =
        user_facing_path.replace("/", "\\");
    if new_path.starts_with("~")
        { new_path = new_path.replacen("~", "%USERPROFILE%", 1) }
    if new_path.starts_with("\\")
        { new_path = new_path.replacen("\\", "%SYSTEMDRIVE%", 1) }

    let mut final_path = if new_path.starts_with('%') {
        let parts: Vec<&str> = new_path.split('%').collect();
        if parts.len() < 2 { String::from(new_path) } else {
            let mut path = win_env_path(parts[1])?;
            for part in &parts[2..] { path.push_str(part); }
            path
        }
    }else { String::from(new_path) };

    // Ensure the path ends with a \\
    if !final_path.ends_with("\\") {
        final_path.push_str("\\");
    }
    while final_path.contains("\\\\") {
        final_path = final_path.replace("\\\\", "\\");
    }
    Ok(final_path)
}

pub fn check_dir_exists(path: &String) -> bool { Path::is_dir(path.as_ref()) }

pub fn get_files_in_dir<P: AsRef<Path>>(path: &P, inc_hidden: &bool) -> io::Result<Vec<FileInfo>> {
    fs::read_dir(path)?.into_iter()
        .filter_map(|entry| {
            let dir = entry.ok()?;
            let meta = dir.metadata().ok()?;
            let file_ext = dir.path().extension()
                .and_then(OsStr::to_str)
                .map(|s| s.to_owned());
            if !inc_hidden && is_hidden(&dir.path()).unwrap(){
                return None
            }
            let file_type = FileInfo::gen_type_enum(&meta.is_dir(),
                                                    &!meta.permissions().readonly(),
                                                    &meta.file_type().is_symlink());
            Some(Ok(FileInfo {
                file_type,
                can_be_written: !meta.permissions().readonly(),
                file_name: dir.file_name(),
                file_ext,
                file_size: meta.len(),
                last_access: meta.accessed().ok(),
                last_modification: meta.modified().ok(),
                creation_time: meta.created().ok(),
                is_hidden: is_hidden(&dir.path()).unwrap()
            }))
        }).collect()
}

// code is_hidden from https://users.rust-lang.org/t/read-windows-hidden-file-attribute/51180/7
pub fn is_hidden(file_path: &std::path::PathBuf) -> io::Result<bool> {
    let metadata = fs::metadata(file_path)?;
    let attributes = metadata.file_attributes();

    if (attributes & 0x2) > 0 {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn format_system_time_opt(sys_time: Option<SystemTime>) -> String{
    sys_time.map(|t|
        DateTime::<Local>::from(t)
            .format("%F %T")
            .to_string())
        .unwrap_or_else(|| "XXXX-XX-XX XX:XX:XX".to_string())
}