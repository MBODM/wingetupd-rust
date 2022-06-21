use crate::{
    app::AppResult,
    errors::{ErrorExtension, UNRECOVERABLE},
};

use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

const PACKAGE_FILE: &str = "packages.txt";
const ERROR_NOT_FOUND: &str = "Package-file not exists.";

pub fn package_file_exists() -> Result<(), String> {
    let package_file_path = get_package_file_path();
    let package_file_exists = Path::new(&package_file_path).exists();
    match package_file_exists {
        true => Ok(()),
        false => Err(ERROR_NOT_FOUND.to_string()),
    }
}

pub fn read_package_file() -> Result<Vec<String>, String> {
    let package_file_path = get_package_file_path();
    let file = File::open(package_file_path).map_err(|err| err.convert(ERROR_NOT_FOUND))?;
    let reader = BufReader::new(file);
    let vec = reader
        .lines()
        .filter_map(std::io::Result::ok)
        .filter(|line| !line.trim().is_empty())
        .collect();
    Ok(vec)
}

fn get_package_file_path() -> &'static str {
    let mut path_buf = env::current_exe().expect(UNRECOVERABLE);
    path_buf.pop();
    path_buf.push(PACKAGE_FILE);
    let path = path_buf.to_str().expect(UNRECOVERABLE);
    path
}
