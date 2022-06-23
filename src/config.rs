use std::{
    env::{self, var_os},
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

const PACKAGE_FILE_NAME: &str = "packages.txt";
const ERROR_NOT_FOUND: &str = "Package-file not exists.";

pub fn get_package_file_path() -> Result<String, String> {
    let exefile_path = get_exefile_path()?;
    if Path::new(&exefile_path).exists() {
        return Ok(exefile_path);
    }
    let appdata_path = get_appdata_path()?;
    if Path::new(&appdata_path).exists() {
        return Ok(appdata_path);
    }
    Err(ERROR_NOT_FOUND.to_string())
}

pub fn package_file_exists() -> Result<(), String> {
    let _ = get_package_file_path()?;
    Ok(())
}

pub fn read_package_file() -> Result<Vec<String>, String> {
    let package_file_path = get_package_file_path()?;
    let file = File::open(package_file_path).map_err(|err| match err.kind() {
        std::io::ErrorKind::NotFound => ERROR_NOT_FOUND.to_string(),
        _ => err.to_string(),
    })?;
    let reader = BufReader::new(file);
    let vec = reader
        .lines()
        .filter_map(std::io::Result::ok)
        .filter(|line| !line.trim().is_empty())
        .collect();
    Ok(vec)
}

fn get_exefile_path() -> Result<String, String> {
    let mut path_buf = env::current_exe().map_err(|err| err.to_string())?;
    path_buf.pop();
    path_buf.push(PACKAGE_FILE_NAME);
    path_buf_to_string(&path_buf)
}

fn get_appdata_path() -> Result<String, String> {
    let mut path_buf = var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .ok_or("Evaluation of %LOCALAPPDATA% environment variable failed.")?;
    path_buf.push("wingetupd");
    path_buf.push(PACKAGE_FILE_NAME);
    path_buf_to_string(&path_buf)
}

fn path_buf_to_string(path_buf: &PathBuf) -> Result<String, String> {
    let s = path_buf
        .into_os_string()
        .into_string()
        .map_err(|_| "Conversion from OsString to String failed.")?;
    Ok(s)
}
