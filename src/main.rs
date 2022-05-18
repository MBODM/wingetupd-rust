use crate::commands::list;

mod commands;
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
    if !winget::installed() {
        return show_error_and_exit("WinGet not installed.");
    }
    if !packages::package_file_exists() {
        return show_error_and_exit("The package-file not exists.");
    }
    let package_id_vec = match packages::read_package_file() {
        Ok(v) => v,
        Err(s) => return show_error_and_exit(&s),
    };
    if package_id_vec.len() == 0 {
        return show_error_and_exit("The package-file is empty.");
    }
    for package_id in package_id_vec.iter() {
        let res = match handle_one_package(&package_id) {
            Ok(r) => r,
            Err(error) => return show_error_and_exit(&error),
        };
    }
    println!("Have a nice day.");
    return EXIT_SUCCESS;
}

fn handle_one_package(package_id: &str) -> Result <PackageInfo, String>
{
    let valid_package = commands::search(&package_id)?;
    if valid_package {
        println!("Valid package: {}", package_id);
    }
    let list_result = commands::list(&package_id)?;
    if list_result.is_installed {
        println!("Installed package: {}", package_id);
    }
    if list_result.is_updatable {
        println!(
            "{} has update: {} -> {}",
            package_id, list_result.installed_version, list_result.update_version
        );
    }
    println!();
    return Ok(PackageInfo
    {
        package_id: package_id.to_string(),
        is_valid: valid_package,
        is_installed: list_result.is_installed,
        is_updatable: list_result.is_updatable,
        installed_version: list_result.installed_version,
        update_version: list_result.update_version,
    });
}

struct PackageInfo
{
    package_id: String,
    is_valid: bool,
    is_installed: bool,
    is_updatable: bool,
    installed_version: String,
    update_version: String,
}

fn show_error_and_exit(error_message: &str) -> i32 {
    println!("Error: {}", error_message);
    return EXIT_FAILURE;
}
