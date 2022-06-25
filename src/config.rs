use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

const PACKAGE_FILE_NAME: &str = "packages.txt";

pub fn package_file_exists() -> bool {
    match get_package_file_path() {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn get_package_file_path() -> Result<String, String> {
    let exefile_path = get_exefile_path()?;
    if Path::new(&exefile_path).exists() {
        return Ok(exefile_path);
    }
    let appdata_path = get_appdata_path()?;
    if Path::new(&appdata_path).exists() {
        return Ok(appdata_path);
    }
    Err("Could not find package-file.".to_string())
}

pub fn read_package_file() -> Result<Vec<String>, String> {
    let package_file_path = get_package_file_path()?;
    let file = File::open(package_file_path).map_err(|_| "Could not open package-file.")?;
    let reader = BufReader::new(file);
    // It´s possible to express this in a more functional manner, by using iterators and closures.
    // But a single line is represented by a Result<> and therefore the functional error handling
    // turns this into something less readable. So i traded some one-liner for better readability.
    let mut packages = vec![];
    let line_results = reader.lines();
    for line_result in line_results {
        let line = match line_result {
            Ok(s) => s,
            Err(e) => return Err("Could not read package-file.".to_string()),
        };
        line = line.trim().to_string();
        if !line.is_empty() {
            packages.push(line);
        }
    }
    Ok(packages)
}

fn get_exefile_path() -> Result<String, String> {
    let mut path_buf =
        env::current_exe().map_err(|_| "Could not determine path to exe file.".to_string())?;
    path_buf.pop();
    path_buf.push(PACKAGE_FILE_NAME);
    path_buf_to_string(&path_buf)
}

fn get_appdata_path() -> Result<String, String> {
    let mut path_buf = env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .ok_or("Could not evaluate %LOCALAPPDATA% environment variable.".to_string())?;
    path_buf.push("MBODM");
    path_buf.push("wingetupd");
    path_buf.push(PACKAGE_FILE_NAME);
    path_buf_to_string(&path_buf)
}

fn path_buf_to_string(path_buf: &PathBuf) -> Result<String, String> {
    let s = path_buf
        .into_os_string()
        .into_string()
        .map_err(|_| "Could not convert OsString to String.".to_string())?;
    Ok(s)
}
