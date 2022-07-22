// Using this newer approach for module declaration, since the
// old mod.rs approach is no longer declared as idiomatic Rust.

// Module externals:

pub use console::{show_goodbye_message, show_title};
pub use domain::run;

// We need no functions here to verify name and version from cargo.toml file,
// since cargo shows some error, if name or version contains an empty string.

pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const AUTHOR: &str = "MBODM";
pub const DATE: &str = "2022-07-22";

// Module internals:

mod args;
mod commands;
mod config;
mod console;
mod domain;
mod packages;
