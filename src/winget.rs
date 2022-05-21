use crate::{app::AppResult, errors::ErrorExtension};
use std::process::Command;

const WINGET_APP: &str = "winget.exe";
const ERROR_NOT_FOUND: &str = "WinGet not installed.";

pub fn installed() -> AppResult<()> {
    Command::new(WINGET_APP)
        .arg("--version")
        .output() /* Not using status(), cause it prints to console. */
        .map_err(|err| err.convert(ERROR_NOT_FOUND))?;
    Ok(())
}

#[derive(Debug)]
pub struct WinGetResult {
    pub process_call: String,
    pub console_output: String,
    pub exit_code: i32,
}

pub fn execute(params: &str) -> AppResult<WinGetResult> {
    let params = params.trim();
    assert!(!params.is_empty());
    let output = Command::new(WINGET_APP)
        .args(params.split(" "))
        .output()
        .map_err(|err| err.convert(ERROR_NOT_FOUND))?;
    let output_string = String::from_utf8(output.stdout)
        .map_err(|_| String::from("WinGet output-format invalid."))?;
    let process_call = format!("{WINGET_APP} {params}");
    let console_output = remove_progressbar_chars(&output_string);
    let exit_code = output
        .status
        .code()
        .ok_or("WinGet not returned any exit code.")?;
    Ok(WinGetResult {
        process_call,
        console_output,
        exit_code,
    })
}

fn remove_progressbar_chars(output_string: &str) -> String {
    output_string
        .trim()
        .replacen(&['\u{0008}', '|', '/', '-', '\\'][..], "", 1)
        .trim()
        .to_string()
}
