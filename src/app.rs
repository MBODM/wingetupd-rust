// We need no functions here to verify name and version from cargo.toml file,
// since cargo shows some error, if name or version contains an empty string.

use crate::{console, args, winget, config};

pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const AUTHOR: &str = "MBODM";
pub const DATE: &str = "2022-05-21";

pub fn run() -> Result<bool, String> {
    println!();
    println!("{NAME} {VERSION} (by {AUTHOR} {DATE})");
    println!();
    if !args::valid() {
        console::show_usage(NAME, false);
        return Ok(false);
    }
    if args::help() {
        console::show_usage(NAME, true);
        return Ok(true);
    }




    // winget::installed()?;
    // config::package_file_exists()?;
    // let packages = config::read_package_file()?;
    // if packages.len() == 0 {
    //     return Err(String::from("Package-file is empty."));
    // }
    //     // console::flush(|| print!("processing ..."));
        // let progress_closure = || console::flush(|| print!("."));
        // let package_infos = core::analyze(packages, progress_closure)?;
        // console::flush(|| print!("... finished."));
        // // let invalid_packages = get_iv_packages(&package_infos);

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

        // println!();
        // for package_info in package_infos.iter() {
        //     let s = &package_info.package;
        //     console::flush(|| println!("{s}"));


    Ok(true)
}

//     let closure = || -> Result<(), String> {
//         winget::installed()?;
//         config::package_file_exists()?;
//         let packages = config::read_package_file()?;
//         if packages.len() == 0 {
//             return Err(String::from("Package-file is empty."));
//         }
//         console::flush(|| print!("processing ..."));
//         let progress_closure = || console::flush(|| print!("."));
//         let package_infos = core::analyze(packages, progress_closure)?;
//         console::flush(|| print!("... finished."));
//         // let invalid_packages = get_iv_packages(&package_infos);

//         // if invalid_packages.len() == 0 {
//         //     console::show_invalid_packages_error(invalid_packages);
//         //     return Err("boing".to_string());
//         console::show_goodby_message();
//         //}

//         // var nonInstalledPackages = packageInfos.Where(packageInfo => !packageInfo.IsInstalled).Select(packageInfo => packageInfo.Package);
//         // if (nonInstalledPackages.Any())
//         // {
//         //     ProgramHelper.ShowNonInstalledPackagesError(nonInstalledPackages);
//         //     Environment.Exit(1);
//         // }

//         println!();
//         for package_info in package_infos.iter() {
//             let s = &package_info.package;
//             console::flush(|| println!("{s}"));

//         Ok(())
//     }
// }
