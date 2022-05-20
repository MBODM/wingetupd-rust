mod commands;
mod config;
mod console;
mod core;
mod errors;
mod parser;
mod winget;

fn main() {
    let exit_code = main_with_exit_code();
    std::process::exit(exit_code);
}

fn main_with_exit_code() -> i32 {
    let closure = || -> Result<(), String> {
        winget::installed()?;
        config::package_file_exists()?;
        let packages = config::read_package_file()?;
        if packages.len() == 0 {
            return Err(String::from("Package-file is empty."));
        }
        console::print("processing ...");
        let progress_closure = || console::print(".");
        let package_infos = core::analyze(packages, progress_closure)?;
        console::print("... finished.");
        console::print_line("");
        for package_info in package_infos.iter() {
            console::print_line(&package_info.package);
        }
        return Ok(());
    };
    match closure() {
        Ok(()) => {
            println!("Have a nice day.");
            return 0;
        }
        Err(err) => {
            println!("Error: {err}");
            return 1;
        }
    };
}
