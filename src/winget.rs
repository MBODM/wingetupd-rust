use std::process::Command;

const WINGET_APP: &str = "winget.exe";

pub fn installed() -> bool {
    match Command::new(WINGET_APP).arg("--version").output() {
        Ok(output) => return output.status.success(),
        Err(_) => return false,
    };
}

pub struct WinGetResultData {
    pub process_call: String,
    pub console_output: String,
    pub exit_code: i32,
}

pub fn run(params: &str) -> Result<WinGetResultData, String> {
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
    let result = WinGetResultData {
        process_call: format!("{} {}", WINGET_APP, params),
        console_output: remove_progressbar_chars(output_string),
        exit_code: output
            .status
            .code()
            .ok_or("WinGet not returned any exit code.")?,
    };
    return Ok(result);
}

fn remove_progressbar_chars(s: String) -> String {
    return s.replace(&['\u{0008}', '|', '/', '-', '\\'][..], "");
}
