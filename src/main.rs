mod commands;
mod parser;
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
    let is_valid = match commands::search("Mozilla.Firefox") {
        Ok(b) => b,
        Err(s) => return show_error_and_exit(&s),
    };
    if is_valid {
        println!("valid");
    }
    let list_result = match commands::list("Mozilla.Firefox") {
        Ok(lr) => lr,
        Err(s) => return show_error_and_exit(&s),
    };
    println!("{:#?}", list_result);
    return EXIT_SUCCESS;
}

fn show_error_and_exit(error_message: &str) -> i32 {
    println!("Error: {}", error_message);
    return EXIT_FAILURE;
}
