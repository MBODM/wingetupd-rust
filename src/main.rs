mod app;
mod args;
mod commands;
mod config;
mod console;
mod core;
mod errors;
mod extensions;
mod parse;
mod prettify;
mod winget;

fn main() {
    let exit_code = run_app();
    std::process::exit(exit_code);
}

fn run_app() -> i32 {
    println!();
    console::show_title();
    println!();
    let exit_code = match app::run() {
        Ok(result) => {
            if result {
                console::show_goodby_message();
            }
            0
        }
        Err(message) => {
            println!("Error: {message}");
            1
        }
    };
    exit_code
}
