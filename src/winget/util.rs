use super::common;

use std::process::Command;

pub fn is_installed() -> bool {
    let winget = common::WINGET_APP;
    // Using .output() here, cause .status() prints to console.
    let result = Command::new(winget).arg("--version").output();
    // If there is NOT a NotFound error, this means WinGet is definitely installed.
    // Even if there is some other error, only NotFound shall produce false result.
    // It is ok to ignore all the other errors and let them happen while executing.
    // The sole purpose here is to determine if the WinGet app is installed or not.
    match result {
        Ok(_) => true,
        Err(error) => !is_notfound_error(&error),
    }
}

pub fn is_notfound_error(error: &std::io::Error) -> bool {
    match error.kind() {
        std::io::ErrorKind::NotFound => true,
        _ => false,
    }
}
