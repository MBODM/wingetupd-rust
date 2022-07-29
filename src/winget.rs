//////////////////////////////////////////////////////////////////
// Using this newer approach for module declaration, since the  //
// old mod.rs approach is no longer declared as idiomatic Rust. //
//////////////////////////////////////////////////////////////////

// Module externals:

pub use common::WinGetError;
pub use exec::{execute, ExecuteData};
pub use parse::{parse_list_output, ParseListOutputData};
pub use prettify::prettify_output;
pub use util::is_installed;

// Module internals:

mod common;
mod exec;
mod parse;
mod prettify;
mod util;
