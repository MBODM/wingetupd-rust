#[derive(Debug, Clone)]
pub struct WinGetError {
    pub message: String,
}

impl WinGetError {
    pub fn new(message: String) -> Self {
        Self { message }
    }

    pub fn new_str(message: &str) -> Self {
        let s = message.to_string();
        Self { message: s }
    }
}

pub const WINGET_APP: &str = "winget.exe";
