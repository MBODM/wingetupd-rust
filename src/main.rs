mod manager;
mod winget;
mod parser;

const EXIT_SUCCESS: i32 = 0;
const EXIT_FAILURE: i32 = 1;

fn main() {
    let exit_code = main_with_exit_code();
    std::process::exit(exit_code);
}

fn main_with_exit_code() -> i32 {
    if !winget::installed() {
        return show_str_and_exit("WinGet not installed.");
    }
    let is_valid = manager::search("Mozilla.Firefox");
    if is_valid {
        let list_result = manager::list("Mozilla.Firefox");
    }

    return EXIT_SUCCESS;
}

fn show_str_and_exit(s: &str) -> i32 {
    println!("Error: {}", s);
    return EXIT_FAILURE;
}
