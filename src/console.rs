use crate::{core::PackageInfo, errors};
use std::io::{stdout, Write};

pub fn flush<T>(print_macro: T)
where
    T: Fn() -> (),
{
    print_macro();
    stdout().flush().expect(errors::UNRECOVERABLE);
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

//         public static void ShowPackageFileEntries(IEnumerable<string> packageFileEntries)
//         {
//             println!($"Found package-file, containing {packageFileEntries.Count()} {EntryOrEntries(packageFileEntries)}.");
//         }

pub fn show_invalid_packages_error(invalid_packages: Vec<&String>) {
    println!("Error: The package-file contains invalid entries.");
    println!();
    println!("The following package-file entries are not valid WinGet package id´s:");
    list_packages(invalid_packages);
    println!();
    println!("You can use 'winget search' to list all valid package id´s.");
    println!();
    println!("Please verify package-file and try again.");
}

pub fn show_non_installed_packages_error(non_installed_packages: Vec<&String>) {
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
    //let valid_packages: Vec<String> = package_infos.iter().filter(|x| x.is_valid).map(|y|&y.package).collect();

    //             var validPackages = packageInfos.Where(packageInfo => packageInfo.IsValid).Select(packageInfo => packageInfo.Package);
    //             var installedPackages = packageInfos.Where(packageInfo => packageInfo.IsInstalled).Select(packageInfo => packageInfo.Package);
    //             var updatablePackages = packageInfos.Where(packageInfo => packageInfo.IsUpdatable);

    //             println!($"{packageInfos.Count()} package-file {EntryOrEntries(packageInfos)} processed.");

    //             println!($"{validPackages.Count()} package-file {EntryOrEntries(packageInfos)} validated.");

    //             println!($"{installedPackages.Count()} {PackageOrPackages(installedPackages)} installed:");
    //             ListPackages(installedPackages);

    //             Console.Write($"{updatablePackages.Count()} {PackageOrPackages(updatablePackages)} updatable");
    //             if (updatablePackages.Any())
    //             {
    //                 println!(":");
    //                 ListUpdateablePackages(updatablePackages);
    //             }
    //             else
    //             {
    //                 println!(".");
    //             }
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

pub fn show_goodby_message() {
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

//         private static string EntryOrEntries<T>(IEnumerable<T> enumerable) =>
//             SingularOrPlural(enumerable, "entry", "entries");

//         private static string PackageOrPackages<T>(IEnumerable<T> enumerable) =>
//             SingularOrPlural(enumerable, "package", "packages");

//         private static string SingularOrPlural<T>(IEnumerable<T> enumerable, string singular, string plural) =>
//             enumerable.Count() == 1 ? singular : plural;

fn list_packages(packages: Vec<&String>) {
    packages
        .iter()
        .for_each(|package| println!("  - {package}"))
}

//         private static void ListPackages(IEnumerable<string> packages) =>
//             packages.ToList().ForEach(package => println!($"  - {package}"));

//         private static void ListUpdateablePackages(IEnumerable<PackageInfo> packageInfos) =>
//             packageInfos.ToList().ForEach(pi => println!($"  - {pi.Package} {pi.InstalledVersion} => {pi.UpdateVersion}"));
//     }
