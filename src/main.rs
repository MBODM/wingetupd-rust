use packages::PackageInfo;

mod app;
mod args;
mod commands;
mod config;
mod console;
mod domain;
mod err;
mod packages;
mod winget;
mod package_infos_helper;

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
    

    let vp = package_infos_helper::get_valid_packages(package_infos);
    let ip = package_infos_helper::get_invalid_packages(package_infos);

    package_infos.push(PackageInfo {
        installed_version: "hans".to_string(),
        is_installed: false,
        is_updatable: false,
        is_valid: false,
        package: "fuzz".to_string(),
        update_version: "wuzz".to_string(),
    });

    let vp1 = package_infos_helper::get_valid_packages(package_infos);
    let ip1 = package_infos_helper::get_invalid_packages(package_infos);
    
    let has_invalid_packages = package_infos.iter().any(|&pi| !pi.is_valid);
    if has_invalid_packages {

        console::show_invalid_packages_error(ip);
        return Err(AppError::new(String::from("Package-file is empty.")));
    }

    //     console::show_invalid_packages_error(invalid_packages);
    //     return Err("boing".to_string());
    console::show_goodbye_message();
    //}

    // var nonInstalledPackages = packageInfos.Where(packageInfo => !packageInfo.IsInstalled).Select(packageInfo => packageInfo.Package);
    // if (nonInstalledPackages.Any())
    // {
    //     ProgramHelper.ShowNonInstalledPackagesError(nonInstalledPackages);
    //     Environment.Exit(1);
    // }

    // println!();
    // for package_info in package_infos.iter() {
    //     let s = &package_info.package;
    //     console::flush(|| println!("{s}"));



    Err(err::AppError::new_str("wuzz"))
}
