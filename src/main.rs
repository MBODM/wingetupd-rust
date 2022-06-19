mod app;
mod args;
mod commands;
mod config;
mod console;
mod core;
mod errors;
mod parse;
mod winget;
mod prettify;

fn main() {
    let exit_code = main_with_exit_code();
    std::process::exit(exit_code);
}

fn main_with_exit_code() -> i32 {
    println!();
    println!(
        "{} {} (by {} {})",
        app::NAME,
        app::VERSION,
        app::AUTHOR,
        app::DATE
    );
    println!();
    if !args::valid() {
        console::show_usage(app::NAME, false);
        return 1;
    }
    if args::help() {
        console::show_usage(app::NAME, true);
        return 0;
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
        // let invalid_packages = get_iv_packages(&package_infos);

        // if invalid_packages.len() == 0 {
            //     console::show_invalid_packages_error(invalid_packages);
            //     return Err("boing".to_string());
            console::show_goodby_message();
        //}

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

// fn get_iv_packages(package_infos: &[core::PackageInfo]) -> &[String] {
//     let s = package_infos
//         .iter()
//         .filter(|&package_info| package_info.is_valid)
//         .map(|package_info| package_info.package.to_string())
//         .collect::<Vec<String>>();
//     let x = s;

//     &x
// }
