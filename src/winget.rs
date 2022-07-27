//////////////////////////////////////////////////////////////////
// Using this newer approach for module declaration, since the  //
// old mod.rs approach is no longer declared as idiomatic Rust. //
//////////////////////////////////////////////////////////////////

// Module externals:

pub use analyze::{analyze_list_output, WinGetAnalyzeResult};
pub use exec::{execute, WinGetExecuteResult};
pub use parse::{parse_list_output, WinGetParseResult};
pub use prettify::prettify_list_output;
pub use util::installed;

// Module internals:

mod analyze;
mod exec;
mod parse;
mod prettify;
mod util;

const WINGET_APP: &str = "winget.exe";
