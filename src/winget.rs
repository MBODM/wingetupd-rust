use std::process::Command;

const WIN_GET_APP: &str = "winget.exe";

pub fn installed() -> bool {
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

// We using expect() for a reason here, instead of using the typical Rust Result<> pattern.

pub fn run(params: &str) -> WinGetResultData {
    let output = Command::new(WIN_GET_APP).args(params.split(" ")).output().expect("winget");
    let string = String::from_utf8(output.stdout).expect("winget");
    return WinGetResultData {
        process_call: format!("{} {}", WIN_GET_APP, params),
        console_output: remove_progressbar_chars(string).trim().to_owned(),
        exit_code: output.status.code().unwrap_or(1),
    };
}

fn remove_progressbar_chars(s: String) -> String {
    return s.replace(&['\u{0008}', '|', '/', '-', '\\'][..], "");
}
