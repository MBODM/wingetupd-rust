// Module 'app' externals:

pub use console::{show_goodbye_message as show_epilog, show_title as show_prolog};
pub use domain::run;

// Module 'app' internals:
mod args;
mod commands;
mod config;
mod console;
mod domain;
mod packages;

// We need no functions here to verify name and version from cargo.toml file,
// since cargo shows some error, if name or version contains an empty string.

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = "MBODM";
const DATE: &str = "2022-05-21";
