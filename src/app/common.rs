#[derive(Debug, Clone)]
pub struct AppError {
    pub message: String,
}

impl AppError {
    pub fn new(message: String) -> Self {
        Self { message }
    }

    pub fn new_str(message: &str) -> Self {
        let s = message.to_string();
        Self { message: s }
    }
}

// We need no functions here to verify name and version from cargo.toml file,
// since cargo shows some error, if name or version contains an empty string.

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const APP_AUTHOR: &str = "MBODM";
pub const APP_DATE: &str = "2022-07-22";
