// Using this newer approach for module declaration, since the
// old mod.rs approach is no longer declared as idiomatic Rust.

// Module externals:

pub use analyze::{analyze_list_output, WinGetAnalyzeResult as Shibby};
pub use exec::{execute_winget as execute, WinGetExecuteResult};
pub use parse::{parse_list_output, WinGetParseResult};
pub use prettify::prettify_list_output;
pub use util::winget_installed as installed;

// Module internals:

mod analyze;
mod exec;
mod parse;
mod prettify;
mod util;
