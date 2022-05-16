use std::error::Error;
use std::process::Command;

const WIN_GET_APP: &str = "winget.exe";

pub fn winget_installed() -> bool {
    match Command::new(WIN_GET_APP).arg("--version").output() {
        Ok(output) => return output.status.success(),
        Err(_) => return false,
    };
}

pub struct WinGetResultData {
    pub process_call: String,
    pub console_output: String,
    pub exit_code: i32,
}

pub fn winget_run(params: &str) -> Result<WinGetResultData, Box<dyn Error>> {
    let output = Command::new(WIN_GET_APP).args(params.split(" ")).output()?;
    let string = String::from_utf8(output.stdout)?;
    let result = WinGetResultData {
        process_call: format!("{} {}", WIN_GET_APP, params),
        console_output: remove_progressbar_chars(string).trim().to_owned(),
        exit_code: output.status.code().unwrap_or(1),
    };
    return Ok(result);
}

fn remove_progressbar_chars(s: String) -> String {
    return s.replace(&['\u{0008}', '|', '/', '-', '\\'][..], "");
}
