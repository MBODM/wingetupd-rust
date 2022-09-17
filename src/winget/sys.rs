use super::WinGetError;

use std::process::Command;

const WINGET_APP: &str = "winget.exe";

pub fn is_installed() -> bool {
    let winget = WINGET_APP;
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

#[derive(Debug, Clone)]
pub struct ExecuteData {
    pub process_call: String,
    pub console_output: String,
    pub exit_code: i32,
}

pub fn execute(params: &str) -> Result<ExecuteData, WinGetError> {
    let params = params.trim();
    assert!(!params.is_empty());
    let winget = WINGET_APP;
    let output = Command::new(winget)
        .args(params.split(" "))
        .output()
        .map_err(|err| match is_notfound_error(&err) {
            // An ErrorKind of NotFound means WinGet is definitely not installed, so use own text.
            // Any other ErrorKind means some another error, so return that error´s specific text.
            true => WinGetError::new_str("WinGet is not installed."),
            false => WinGetError::new(err.to_string()),
        })?;
    let process_call = format!("{winget} {params}");
    let console_output = String::from_utf8(output.stdout)
        .map_err(|_| WinGetError::new_str("WinGet output format is invalid."))?;
    let exit_code = output
        .status
        .code()
        .ok_or(WinGetError::new_str("WinGet not returned any exit code."))?;
    Ok(ExecuteData {
        process_call,
        console_output,
        exit_code,
    })
}

fn is_notfound_error(error: &std::io::Error) -> bool {
    match error.kind() {
        std::io::ErrorKind::NotFound => true,
        _ => false,
    }
}
