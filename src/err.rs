#[derive(Debug, Clone)]
pub struct AppError {
    pub msg: String,
}

impl AppError {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }

    pub fn new_str(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }
}
