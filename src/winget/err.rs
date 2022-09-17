#[derive(Debug, Clone)]
pub struct WinGetError {
    pub msg: String,
}

impl WinGetError {
    // It is possible to just use &str here and rely on deref coercion, when calling the function.
    // This is heavily discussed, but i stick with the "fn should express its intension" approach.

    pub fn new(msg: String) -> Self {
        Self { msg }
    }

    pub fn new_str(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }
}
