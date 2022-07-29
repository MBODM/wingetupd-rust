//////////////////////////////////////////////////////////////////
// Using this newer approach for module declaration, since the  //
// old mod.rs approach is no longer declared as idiomatic Rust. //
//////////////////////////////////////////////////////////////////

// Module externals:

pub use console::{show_goodbye_message, show_title};
pub use domain::run;

// Module internals:

mod args;
mod commands;
mod common;
mod config;
mod console;
mod domain;
mod packages;
