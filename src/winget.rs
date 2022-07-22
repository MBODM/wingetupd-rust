// Using this newer approach for module declaration, since the
// old mod.rs approach is no longer declared as idiomatic Rust.

// Module externals:

pub use analyze::{analyze_list_output as analyze, WinGetAnalyzeResult as Shibby};
pub use exec::{execute_winget as execute, winget_installed as installed, WinGetExecuteResult};
pub use parse::{parse_list_output as parse, WinGetParseResult};
pub use prettify::prettify_list_output as prettify;

// Module internals:

mod analyze;
mod exec;
mod parse;
mod prettify;
