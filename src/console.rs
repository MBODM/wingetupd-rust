use super::{app, packages::PackageInfo};

use std::{
    intrinsics::const_eval_select,
    io::{stdout, Write},
};

pub fn flush<T>(print_macro: T)
where
    T: Fn() -> (),
{
    print_macro();
    stdout().flush().expect("todo")
}

pub fn show_title() {
    println!(
        "{} {} (by {} {})",
        app::NAME,
        app::VERSION,
        app::AUTHOR,
        app::DATE
    );
}

pub fn show_usage(exe_file: &str, is_help: bool) {
    if !is_help {
        println!("Error: Unknown parameter(s).");
        println!();
    }
    println!("Usage: {exe_file} [--no-log] [--no-confirm]");
    println!();
    println!("  --no-log      Don´t create log file (useful when running from a folder without write permissions)");
    println!("  --no-confirm  Don´t ask for update confirmation (useful for script integration)");
    println!();
    println!(
        "For more information have a look at the GitHub page (https://github.com/MBODM/wingetupd"
    );
}

pub fn show_package_file_entries(package_file_entries: Vec<String>) {
    let i = package_file_entries.len();
    let s = entry_or_entries(package_file_entries);
    println!("Found package-file, containing {i} {s}.");
}

pub fn show_invalid_packages_error(invalid_packages: Vec<String>) {
    println!("Error: The package-file contains invalid entries.");
    println!();
    println!("The following package-file entries are not valid WinGet package id´s:");
    list_packages(invalid_packages);
    println!();
    println!("You can use 'winget search' to list all valid package id´s.");
    println!();
    println!("Please verify package-file and try again.");
}

pub fn show_non_installed_packages_error(non_installed_packages: Vec<String>) {
    println!("Error: The package-file contains non-installed packages.");
    println!();
    println!("The following package-file entries are valid WinGet package id´s,");
    println!("but those packages are not already installed on this machine yet:");
    list_packages(non_installed_packages);
    println!();
    println!("You can use 'winget list' to show all installed packages and their package id´s.");
    println!();
    println!("Please verify package-file and try again.");
}

pub fn show_summary(package_infos: Vec<PackageInfo>) {
    let i = package_infos.len();
    let s = entry_or_entries(package_infos);
    println!("{i} package-file {s} processed.");
    let valid_packages = package_infos
        .iter()
        .filter(|&&pi| pi.is_valid)
        .map(|&pi| pi.package)
        .collect::<Vec<String>>();
    let i = valid_packages.len();
    let s = entry_or_entries(valid_packages);
    println!("{i} package-file {s} validated.");
    let installed_packages = package_infos
        .iter()
        .filter(|&&pi| pi.is_installed)
        .map(|&pi| pi.package)
        .collect::<Vec<String>>();
    let i = installed_packages.len();
    let s = package_or_packages(installed_packages);
    println!("{i} {s} installed:");
    list_packages(installed_packages);
    let updatable_packages = package_infos
        .iter()
        .filter(|&&pi| pi.is_updatable)
        .collect::<Vec<PackageInfo>>();
    let i = updatable_packages.len();
    let s = package_or_packages(updatable_packages);
    println!("{i} {s} updatable");
    if updatable_packages.len() != 0 {
        println!(":");
        list_updateable_packages(updatable_packages);
    } else {
        println!(".");
    }
}

//         public static bool AskUpdateQuestion(IEnumerable<string> updateablePackages)
//         {
//             Console.Write($"Update {updateablePackages.Count()} {PackageOrPackages(updateablePackages)} ? [y/N]: ");

//             while (true)
//             {
//                 var key = Console.ReadKey(true).Key;

//                 if (key == ConsoleKey.N || key == ConsoleKey.Enter)
//                 {
//                     Console.Write("N");
//                     println!();
//                     println!();

//                     return false;
//                 }

//                 if (key == ConsoleKey.Y)
//                 {
//                     Console.Write("y");
//                     println!();
//                     println!();

//                     return true;
//                 }
//             }
//         }

//         public static void ShowUpdatedPackages(IEnumerable<string> updatedPackages)
//         {
//             println!();
//             println!($"{updatedPackages.Count()} {PackageOrPackages(updatedPackages)} updated:");

//             ListPackages(updatedPackages);
//         }

pub fn show_goodbye_message() {
    println!("Have a nice day.");
}

//         public static void ShowWinGetError(string error, string log)
//         {
//             var winGetLogFolder = Path.Combine(
//                 Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData),
//                 @"Packages\Microsoft.DesktopAppInstaller_8wekyb3d8bbwe\LocalState\DiagOutputDir");

//             var winGetWebSite = "https://docs.microsoft.com/en-us/windows/package-manager/winget";

//             println!();
//             println!();
//             println!($"Error: {error}");
//             println!();
//             println!($"For details have a look at the log file ('{log}').");
//             println!();
//             println!("For even more details have a look at WinGet´s own log files:");
//             println!(winGetLogFolder);
//             println!();
//             println!("You can also find further information on the WinGet site: ");
//             println!(winGetWebSite);
//         }

fn entry_or_entries<T>(vec: Vec<T>) -> String {
    singular_or_plural(vec, "entry", "entries")
}

fn package_or_packages<T>(vec: Vec<T>) -> String {
    singular_or_plural(vec, "package", "packages")
}

fn singular_or_plural<T>(vec: Vec<T>, singular: &str, plural: &str) -> String {
    match vec.len() == 1 {
        true => singular.to_string(),
        false => plural.to_string(),
    }
}

fn list_packages(packages: Vec<String>) {
    packages
        .iter()
        .for_each(|&package| println!("  - {package}"))
}

fn list_updateable_packages(package_infos: Vec<PackageInfo>) {
    package_infos.iter().for_each(|&pi| {
        let package = pi.package;
        let installed_version = pi.installed_version;
        let update_version = pi.update_version;
        println!("  - {package} {installed_version} => {update_version}");
    });
}
