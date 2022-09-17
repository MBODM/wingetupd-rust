//////////////////////////////////////////////////////////////////
// Using this newer approach for module declaration, since the  //
// old mod.rs approach is no longer declared as idiomatic Rust. //
//////////////////////////////////////////////////////////////////

// Module externals:

pub use err::WinGetError;
pub use parse::{parse_list_output, ParseListOutputData};
pub use prettify::prettify_output;
pub use sys::{execute, is_installed, ExecuteData};

// Module internals:

mod err;
mod parse;
mod prettify;
mod sys;
