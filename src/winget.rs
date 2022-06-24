use std::process::Command;

const WINGET_APP: &str = "winget.exe";
const ERROR_NOT_FOUND: &str = "WinGet not installed.";

pub fn installed() -> Result<(), String> {
    Command::new(WINGET_APP)
        .arg("--version")
        .output() /* Not using status(), cause it prints to console. */
        .map_err(|err| get_err_msg(err))?;
    Ok(())
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
        .map_err(|err| get_err_msg(err))?;
    let process_call = format!("{WINGET_APP} {params}");
    let console_output = String::from_utf8(output.stdout).map_err(|_| "WinGet output invalid.")?;
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

fn get_err_msg(error: std::io::Error) -> String {
    match error.kind() {
        std::io::ErrorKind::NotFound => ERROR_NOT_FOUND.to_string(),
        _ => error.to_string(),
    }
}
