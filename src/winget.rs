use std::io;
use std::process::Command;

const WINGET_APP: &str = "winget.exe";

pub fn installed() -> Result<(), String> {
    Command::new(WINGET_APP)
        .arg("--version")
        .output()
        .map_err(|err| convert_io_error(err))?;
    return Ok(());
}

#[derive(Debug)]
pub struct WinGetResult {
    pub process_call: String,
    pub console_output: String,
    pub exit_code: i32,
}

pub fn execute(params: &str) -> Result<WinGetResult, String> {
    let params = params.trim();
    assert!(!params.is_empty());
    let output = Command::new(WINGET_APP)
        .args(params.split(" "))
        .output()
        .map_err(|err| convert_io_error(err))?;
    let output_string = String::from_utf8(output.stdout)
        .map_err(|_| String::from("WinGet output-format invalid."))?
        .trim()
        .to_string();
    let process_call = build_process_call(params);
    let console_output = remove_progressbar_chars(output_string);
    let exit_code = output
        .status
        .code()
        .ok_or("WinGet not returned any exit code.")?;
    return Ok(WinGetResult {
        process_call,
        console_output,
        exit_code,
    });
}

fn convert_io_error(err: io::Error) -> String {
    return match err.kind() {
        io::ErrorKind::NotFound => String::from("WinGet not installed."),
        _ => err.to_string(),
    };
}

fn build_process_call(params: &str) -> String {
    return format!("{WINGET_APP} {params}");
}

fn remove_progressbar_chars(s: String) -> String {
    return s.replace(&['\u{0008}', '|', '/', '-', '\\'][..], "");
}
