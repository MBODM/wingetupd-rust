pub const UNRECOVERABLE: &str = "[WinGetUpd - Unrecoverable application error]";

pub type AppError = String;

pub trait ErrorExtension {
    fn convert(&self, custom_not_found_message: &str) -> String;
}

impl ErrorExtension for std::io::Error {
    fn convert(&self, custom_not_found_message: &str) -> String {
        match self.kind() {
            std::io::ErrorKind::NotFound => custom_not_found_message.to_string(),
            _ => self.to_string(),
        }
    }
}
