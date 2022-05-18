use std::process::Command;

const WINGET_APP: &str = "winget.exe";
const ERROR_NO_EXITCODE: &str = "WinGet not returned any exit code.";

pub fn installed() -> bool {
    match Command::new(WINGET_APP).arg("--version").output() {
        Ok(output) => return output.status.success(),
        Err(_) => return false,
    };
}

#[derive(Debug)]
pub struct RunResult {
    pub process_call: String,
    pub console_output: String,
    pub exit_code: i32,
}

pub fn run(params: &str) -> Result<RunResult, String> {
    let params = params.trim();
    assert!(!params.is_empty());
    let output = Command::new(WINGET_APP)
        .args(params.split(" "))
        .output()
        .map_err(|err| err.to_string())?;
    let output_string = String::from_utf8(output.stdout)
        .map_err(|err| err.to_string())?
        .trim()
        .to_string();
    let process_call = create_process_call(params);
    let console_output = remove_progressbar_chars(output_string);
    let exit_code = output.status.code().ok_or(ERROR_NO_EXITCODE)?;
    return Ok(RunResult {
        process_call,
        console_output,
        exit_code,
    });
}

fn create_process_call(params: &str) -> String {
    return format!("{} {}", WINGET_APP, params);
}

fn remove_progressbar_chars(s: String) -> String {
    return s.replace(&['\u{0008}', '|', '/', '-', '\\'][..], "");
}
