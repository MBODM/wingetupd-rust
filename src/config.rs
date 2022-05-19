use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

const PACKAGE_FILE: &str = "packages.txt";

pub fn package_file_exists() -> bool {
    let package_file = get_package_file_path();
    return Path::new(&package_file).exists();
}

pub fn read_package_file() -> Result<Vec<String>, String> {
    let package_file = get_package_file_path();
    let file = File::open(package_file).map_err(|err| err.to_string())?;
    let reader = BufReader::new(file);
    let vec = reader
        .lines()
        .filter_map(io::Result::ok)
        .filter(|line| !line.trim().is_empty())
        .collect();
    return Ok(vec);
}

fn get_package_file_path() -> String {
    // It´s ok to unwrap here, because if this
    // ever happens, it´s really time to panic! :)
    let mut path_buf = env::current_exe().unwrap();
    path_buf.pop();
    path_buf.push(PACKAGE_FILE);
    let package_file_path = path_buf.into_os_string().into_string().unwrap();
    return package_file_path;
}
