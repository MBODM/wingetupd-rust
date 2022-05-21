pub type AppResult<T> = Result<T, String>;

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const APP_AUTHOR: &str = "MBODM";
pub const APP_DATE: &str = "2022-05-21";
