use crate::errors::{ErrorExtension, UNRECOVERABLE};
use std::{
    env::{self, var_os},
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

const PACKAGE_FILE_NAME: &str = "packages.txt";
const ERROR_NOT_FOUND: &str = "Package-file not exists.";

pub fn get_package_file_path() -> Result<String, String> {
    let exefile_path = get_exefile_path();
    if Path::new(&exefile_path).exists() {
        return Ok(exefile_path);
    }
    let appdata_path = get_appdata_path();
    if Path::new(&appdata_path).exists() {
        return Ok(appdata_path);
    }
    Err(ERROR_NOT_FOUND.to_string())
}

pub fn package_file_exists() -> bool {
    match get_package_file_path() {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn read_package_file() -> Result<Vec<String>, String> {
    let package_file_path = get_package_file_path()?;
    let file = File::open(package_file_path).map_err(|err| err.convert(ERROR_NOT_FOUND))?;
    let reader = BufReader::new(file);
    let vec = reader
        .lines()
        .filter_map(std::io::Result::ok)
        .filter(|line| !line.trim().is_empty())
        .collect();
    Ok(vec)
}

fn get_exefile_path() -> String {
    let mut path_buf = env::current_exe().expect(UNRECOVERABLE);
    path_buf.pop();
    path_buf.push(PACKAGE_FILE_NAME);
    path_buf_to_string(&path_buf)
}

fn get_appdata_path() -> String {
    let mut path_buf = var_os("APPDATA").map(PathBuf::from).expect(UNRECOVERABLE);
    path_buf.push("wingetupd");
    path_buf.push(PACKAGE_FILE_NAME);
    path_buf_to_string(&path_buf)
}

fn path_buf_to_string(path_buf: &PathBuf) -> String {
    path_buf
        .into_os_string()
        .into_string()
        .expect(UNRECOVERABLE)
}
