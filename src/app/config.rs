use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use super::common::AppError;

const PACKAGE_FILE_NAME: &str = "packages.txt";

pub fn package_file_exists() -> bool {
    match get_package_file_path() {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn get_package_file_path() -> Result<String, AppError> {
    let exefile_path = get_exefile_path()?;
    if Path::new(&exefile_path).exists() {
        return Ok(exefile_path);
    }
    let appdata_path = get_appdata_path()?;
    if Path::new(&appdata_path).exists() {
        return Ok(appdata_path);
    }
    Err(AppError::new_str("Could not find package-file."))
}

pub fn read_package_file() -> Result<Vec<String>, AppError> {
    let package_file_path = get_package_file_path()?;
    let file = File::open(package_file_path)
        .map_err(|_| AppError::new_str("Could not open package-file."))?;
    let reader = BufReader::new(file);
    // It´s possible to express this in a more functional manner, by using iterators and closures.
    // But a single line is represented by a Result<> and therefore the functional error handling
    // turns this into something less readable. So i traded some one-liner for better readability.
    let mut packages = vec![];
    let line_results = reader.lines();
    let mut line: String;
    for line_result in line_results {
        line = match line_result {
            Ok(s) => s,
            Err(_) => return Err(AppError::new_str("Could not read package-file.")),
        };
        line = line.trim().to_string();
        if !line.is_empty() {
            packages.push(line);
        }
    }
    Ok(packages)
}

fn get_exefile_path() -> Result<String, AppError> {
    let mut path_buf = env::current_exe()
        .map_err(|_| AppError::new_str("Could not determine path to exe file."))?;
    path_buf.pop();
    path_buf.push(PACKAGE_FILE_NAME);
    path_buf_to_string(&path_buf)
}

fn get_appdata_path() -> Result<String, AppError> {
    let mut path_buf = env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .ok_or(AppError::new_str(
            "Could not evaluate %LOCALAPPDATA% environment variable.",
        ))?;
    path_buf.push("MBODM");
    path_buf.push("wingetupd");
    path_buf.push(PACKAGE_FILE_NAME);
    path_buf_to_string(&path_buf)
}

fn path_buf_to_string(path_buf: &PathBuf) -> Result<String, AppError> {
    let s = path_buf
        .into_os_string()
        .into_string()
        .map_err(|_| AppError::new_str("Could not convert OsString to String."))?;
    Ok(s)
}
