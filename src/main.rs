mod app;
mod args;
mod commands;
mod config;
mod console;
mod domain;
mod err;
mod packages;
mod winget;

// How to gracefully exit a rust app with some exit code? See here:
// https://stackoverflow.com/questions/30281235/how-to-cleanly-end-the-program-with-an-exit-code
// https://stackoverflow.com/questions/24245276/why-does-rust-not-have-a-return-value-in-the-main-function-and-how-to-return-a

fn main() -> Result<(), ()> {
    println!();
    console::show_title();
    println!();
    match run() {
        Ok(show_goodbye_message) => {
            if show_goodbye_message {
                console::show_goodbye_message();
            }
            // Since Rust 1.26 this leads to platform-specific EXIT_SUCCESS:
            Ok(())
        }
        Err(app_error) => {
            let msg = app_error.msg;
            println!("Error: {msg}");
            // Since Rust 1.26 this leads to platform-specific EXIT_FAILURE:
            Err(())
        }
    }
}

fn run() -> Result<bool, err::AppError> {
    if !args::valid() {
        console::show_usage(app::NAME, false);
        return Ok(false);
    }
    if args::has_help() {
        console::show_usage(app::NAME, true);
        return Ok(true);
    }
    if !winget::is_installed() {
        return Err(err::AppError::new_str("TODO: winget not installed."));
    }
    // if !config::package_file_exists() {
    //     return Err("TODO: No package file found.".to_string());
    // }
    let packages = config::read_package_file()?;
    if packages.len() == 0 {
        return Err(err::AppError::new(String::from("Package-file is empty.")));
    }
    console::flush(|| print!("processing ..."));
    let progress_closure = || console::flush(|| print!("."));
    let package_infos = packages::analyze(packages, progress_closure)?;
    console::flush(|| print!("... finished."));

    Err(err::AppError::new_str("wuzz"))
}
