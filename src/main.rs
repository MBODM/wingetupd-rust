mod winget;
mod manager;

const EXIT_SUCCESS: i32 = 0;
const EXIT_FAILURE: i32 = 1;
const ERROR_WINGET_NOT_INSTALLED: &str = "WinGet not installed.";

fn main() {
    let exit_code = main_with_exit_code();
    std::process::exit(exit_code);
}

fn main_with_exit_code() -> i32 {
    if !winget::winget_installed() {
        return show_str_and_exit(ERROR_WINGET_NOT_INSTALLED);
    }
    let params = "search--exact --id Mozilla.Firefox";
    let result = winget::winget_run(params);
    let data = match result {
        Ok(d) => d,
        Err(e) => return show_error_and_exit(e),
    };
    if data.exit_code != 0 {
        return show_str_and_exit("stinkt");
    }
    println!("--------------------------------------------------");
    println!("process call:");
    println!("{}", data.process_call);
    println!("--------------------------------------------------");
    println!("console output:");
    println!("{}", data.console_output);
    println!("--------------------------------------------------");
    println!("exit code:");
    println!("{}", data.exit_code);
    println!("--------------------------------------------------");
    return EXIT_SUCCESS;
}

fn show_str_and_exit(s: &str) -> i32 {
    println!("Error: {}", s);
    return EXIT_FAILURE;
}

fn show_error_and_exit(e: Box<dyn std::error::Error>) -> i32 {
    println!("Error: {}", e);
    return EXIT_FAILURE;
}
