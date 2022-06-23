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
pub trait PathBufExtension {
    fn to_string(&self) -> String;
}

impl PathBufExtension for std::path::PathBuf {
    fn to_string(&self) -> String {
        self.into_os_string().to_os_string()
    }
}
