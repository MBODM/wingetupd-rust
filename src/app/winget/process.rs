use std::{io::ErrorKind, process::Command};

const WINGET_APP: &str = "winget.exe";

pub fn installed() -> bool {
    // Using .output() here, cause .status() prints to console.
    let result = Command::new(WINGET_APP).arg("--version").output();
    // If there is NOT a NotFound error, this means WinGet is definitely installed.
    // Even if there is some other error, only NotFound shall produce false result.
    // It is ok to ignore all the other errors and let them happen in the fn below.
    // The sole purpose here is to determine if the WinGet app is installed or not.
    match result {
        Ok(_) => true,
        Err(error) => !is_not_found_error(error),
    }
}

#[derive(Debug)]
pub struct WinGetExecuteResult {
    pub process_call: String,
    pub console_output: String,
    pub exit_code: i32,
}

pub fn execute(params: &str) -> Result<WinGetExecuteResult, String> {
    let params = params.trim();
    assert!(!params.is_empty());
    let output = Command::new(WINGET_APP)
        .args(params.split(" "))
        .output()
        .map_err(|err| match is_not_found_error(err) {
            true => "WinGet is not installed.".to_string(),
            false => err.to_string(),
        })?;
    let process_call = format!("{WINGET_APP} {params}");
    let console_output =
        String::from_utf8(output.stdout).map_err(|_| "WinGet output format is invalid.")?;
    let exit_code = output
        .status
        .code()
        .ok_or("WinGet not returned any exit code.")?;
    Ok(WinGetExecuteResult {
        process_call,
        console_output,
        exit_code,
    })
}

fn is_not_found_error(error: std::io::Error) -> bool {
    match error.kind() {
        ErrorKind::NotFound => true,
        _ => false,
    }
}
