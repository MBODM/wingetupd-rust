use app::AppResult;

use crate::args::valid;

mod app;
mod args;
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
    let app_name = app::APP_NAME;
    let app_version = app::APP_VERSION;
    let app_author = app::APP_AUTHOR;
    let app_date = app::APP_DATE;
    println!();
    println!("{app_name} {app_version} (by {app_author} {app_date})");
    println!();
    if !args::valid() {
        console::show_usage(app_name, !args::has_help());
        return 1;
    }
    let closure = || -> Result<(), String> {
        winget::installed()?;
        config::package_file_exists()?;
        let packages = config::read_package_file()?;
        if packages.len() == 0 {
            return Err(String::from("Package-file is empty."));
        }
        console::flush(|| print!("processing ..."));
        let progress_closure = || console::flush(|| print!("."));
        let package_infos = core::analyze(packages, progress_closure)?;
        console::flush(|| print!("... finished."));
        if check_valid_packages(package_infos) {
            console::show_invalid_packages_error(invalid_packages);
            return Err("boing".to_string());
        }

        // var nonInstalledPackages = packageInfos.Where(packageInfo => !packageInfo.IsInstalled).Select(packageInfo => packageInfo.Package);
        // if (nonInstalledPackages.Any())
        // {
        //     ProgramHelper.ShowNonInstalledPackagesError(nonInstalledPackages);
        //     Environment.Exit(1);
        // }

        println!();
        for package_info in package_infos.iter() {
            let s = &package_info.package;
            console::flush(|| println!("{s}"));
        }
        Ok(())
    };
    match closure() {
        Ok(()) => {
            console::show_goodby_message();
            0
        }
        Err(err) => {
            println!("Error: {err}");
            1
        }
    }
}

fn check_valid_packages(package_infos: Vec<&String>) -> bool {
    package_infos
        .iter()
        .filter(|package_info| !package_info.is_valid)
        .map(|package_info| &package_info.package)
        .collect()
        .len()
        > 0
}
