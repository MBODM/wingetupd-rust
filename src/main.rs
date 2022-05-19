mod commands;
mod config;
mod core;
mod helper;
mod parser;
mod printer;
mod summary;
mod winget;

const EXIT_SUCCESS: i32 = 0;
const EXIT_FAILURE: i32 = 1;

fn main() {
    let exit_code = main_with_exit_code();
    std::process::exit(exit_code);
}

fn main_with_exit_code() -> i32 {
    if !winget::installed() {
        return show_error_and_exit("WinGet not installed.");
    }
    if !config::package_file_exists() {
        return show_error_and_exit("The package-file not exists.");
    }
    let packages = match config::read_package_file() {
        Ok(vec) => vec,
        Err(err) => return show_error_and_exit(&err),
    };
    if packages.len() == 0 {
        return show_error_and_exit("The package-file is empty.");
    }
    helper::console_write("processing ...");
    let progress_closure = || helper::console_write(".");
    let package_infos = match core::analyze(packages, progress_closure) {
        Ok(pi) => pi,
        Err(err) => return show_error_and_exit(&err),
    };
    helper::console_write("... finished.");
    for package_info in package_infos.iter() {
        println!("{}", package_info.package_id);
    }
    println!("Have a nice day.");
    return EXIT_SUCCESS;
}

fn show_error_and_exit(error_message: &str) -> i32 {
    println!("Error: {}", error_message);
    return EXIT_FAILURE;
}
