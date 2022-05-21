use std::io::{Error, ErrorKind};

pub const UNRECOVERABLE: &str = "[unrecoverable application error]";

pub trait ErrorExtension {
    fn convert(&self, not_found_msg: &str) -> String;
}

impl ErrorExtension for Error {
    fn convert(&self, not_found_msg: &str) -> String {
        match self.kind() {
            ErrorKind::NotFound => String::from(not_found_msg),
            _ => self.to_string(),
        }
    }
}
