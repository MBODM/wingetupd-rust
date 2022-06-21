mod app;
mod args;
mod commands;
mod config;
mod console;
mod core;
mod errors;
mod parse;
mod prettify;
mod winget;

fn main() {
  
    match app::run() {
        Ok(result) => {
            if result {
                console::show_goodby_message();
            }
            std::process::exit(0);
        }
        Err(err) => {
            println!("Error: {err}");
            std::process::exit(1);
        }
    }
}
