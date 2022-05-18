mod packages;
mod parser;
mod winget;

const EXIT_SUCCESS: i32 = 0;
const EXIT_FAILURE: i32 = 1;

fn main() {
    let exit_code = main_with_exit_code();
    std::process::exit(exit_code);
}

fn main_with_exit_code() -> i32 {
    parser::parse_winget_list_output("fuzz");
    return 0;
    if !winget::installed() {
        return show_error_and_exit("WinGet not installed.");
    }
    let is_valid = match packages::search("ozilla.Firefox") {
        Ok(b) => b,
        Err(s) => return show_error_and_exit(s.as_str()),
    };
    if (is_valid) {
        println!("valid");
    }
    return EXIT_SUCCESS;
}

fn show_error_and_exit(error_message: &str) -> i32 {
    println!("Error: {}", error_message);
    return EXIT_FAILURE;
}
