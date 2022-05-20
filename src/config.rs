use crate::errors;
use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

const PACKAGE_FILE: &str = "packages.txt";
const ERROR_NOT_FOUND: &str = "Package-file not found.";

pub fn package_file_exists() -> Result<(), String> {
    let package_file = get_package_file_path();
    let package_file_exists = Path::new(&package_file).exists();
    return match package_file_exists {
        true => Ok(()),
        false => Err(String::from(ERROR_NOT_FOUND)),
    };
}

pub fn read_package_file() -> Result<Vec<String>, String> {
    let package_file = get_package_file_path();
    let file = File::open(package_file).map_err(|err| convert_io_error(err))?;
    let reader = BufReader::new(file);
    let vec = reader
        .lines()
        .filter_map(io::Result::ok)
        .filter(|line| !(line.trim().is_empty()))
        .collect();
    return Ok(vec);
}

fn get_package_file_path() -> String {
    let mut path_buf = env::current_exe().expect(errors::UNRECOVERABLE);
    path_buf.pop();
    path_buf.push(PACKAGE_FILE);
    return path_buf
        .into_os_string()
        .into_string()
        .expect(errors::UNRECOVERABLE);
}

fn convert_io_error(err: io::Error) -> String {
    return match err.kind() {
        io::ErrorKind::NotFound => String::from(ERROR_NOT_FOUND),
        _ => err.to_string(),
    };
}
