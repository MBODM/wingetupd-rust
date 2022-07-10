// Module 'winget' externals:

pub use parse::{parse_list_output, WinGetParseResult};
pub use prettify::prettify_output;
pub use process::{execute, installed, WinGetExecuteResult};

// Module 'winget' internals:

mod parse;
mod prettify;
mod process;
