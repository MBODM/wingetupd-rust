use std::process::Command;

#[derive(Debug)]
pub struct WinGetExecuteResult {
    pub process_call: String,
    pub console_output: String,
    pub exit_code: i32,
}

pub fn execute(params: &str) -> Result<WinGetExecuteResult, String> {
    let params = params.trim();
    assert!(!params.is_empty());
    let app = super::WINGET_APP;
    let output = Command::new(app)
        .args(params.split(" "))
        .output()
        .map_err(|err| match super::util::is_not_found_error(err) {
            true => "WinGet is not installed.".to_string(),
            false => err.to_string(),
        })?;
    let process_call = format!("{app} {params}");
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
