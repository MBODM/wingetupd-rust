use super::exec;
use std::{io::ErrorKind, process::Command};

pub fn winget_installed() -> bool {
    // Using .output() here, cause .status() prints to console.
    let result = Command::new(exec::WINGET_APP).arg("--version").output();
    // If there is NOT a NotFound error, this means WinGet is definitely installed.
    // Even if there is some other error, only NotFound shall produce false result.
    // It is ok to ignore all the other errors and let them happen in the fn below.
    // The sole purpose here is to determine if the WinGet app is installed or not.
    match result {
        Ok(_) => true,
        Err(error) => !is_not_found_error(error),
    }
}

pub fn is_not_found_error(error: std::io::Error) -> bool {
    match error.kind() {
        ErrorKind::NotFound => true,
        _ => false,
    }
}
