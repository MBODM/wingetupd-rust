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
const ERROR_NOT_FOUND: &str = "Package-file not found.";

pub fn package_file_exists() -> AppResult<()> {
    let package_file = get_package_file_path();
    let package_file_exists = Path::new(&package_file).exists();
    match package_file_exists {
        true => Ok(()),
        false => Err(String::from(ERROR_NOT_FOUND)),
    }
}

pub fn read_package_file() -> AppResult<Vec<String>> {
    let package_file = get_package_file_path();
    let file = File::open(package_file).map_err(|err| err.convert(ERROR_NOT_FOUND))?;
    let reader = BufReader::new(file);
    let vec = reader
        .lines()
        .filter_map(std::io::Result::ok)
        .filter(|line| !line.trim().is_empty())
        .collect();
    Ok(vec)
}

fn get_package_file_path() -> String {
    let mut path_buf = env::current_exe().expect(UNRECOVERABLE);
    path_buf.pop();
    path_buf.push(PACKAGE_FILE);
    path_buf
        .into_os_string()
        .into_string()
        .expect(UNRECOVERABLE)
}
