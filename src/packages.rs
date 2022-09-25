use super::{commands, err::AppError};


type MySlice<'a, T> = &'a[T];
type StringSlice<'a> = &'a[String];
type PackageInfoSlice<'a> = &'a[PackageInfo];


#[derive(Debug)]
pub struct PackageInfo {
    pub package: String,
    pub is_valid: bool,
    pub is_installed: bool,
    pub is_updatable: bool,
    pub installed_version: String,
    pub update_version: String,
}

pub fn analyze<T>(packages_slice: &[String], progress_handler: T) -> Result<Vec<PackageInfo>, AppError>
where
    T: Fn() -> (),
{
    packages_slice
        .iter()
        .map(|&package| {
            let valid = commands::search(package.as_str())?;
            progress_handler();
            if !valid {
                return Ok(PackageInfo{
                    package,
                    is_valid: false,
                    is_installed: false,
                    is_updatable: false,
                    installed_version: "".to_string(),
                    update_version: "".to_string(),
                });
            }
            let list_data = commands::list(package.as_str())?;
            progress_handler();
            Ok(PackageInfo {
                package,
                is_valid: valid,
                is_installed: list_data.is_installed,
                is_updatable: list_data.is_updatable,
                installed_version: list_data.installed_version,
                update_version: list_data.update_version,
            })
        })
        .collect()
}

// Of course it´s normally better to use slices as arguments, instead of a Vec, if
// you don´t need to add or remove elements. But since it doesn´t matter much here
// performance-wise and the syntax looks easier for beginners, in my opinion, i go
// with a Vec here. Also i may do some reference copying (or stuff like that) here,
// which is unnecessary. But since i´m also a Rust beginner and need to understand
// what´s going on here, in a year from now, without much Rust coding meanwhile, i
// don´t want to be that complex syntax-wise, just for some 0.0000001% performance.

pub fn get_valid_packages(package_infos: MySlice<PackageInfo>) -> Vec<String>
{
    package_infos.iter().filter(|&&pi| pi.is_valid).map(|&pi| pi.package).collect()
}

pub fn get_invalid_packages(package_infos: Vec<PackageInfo>) -> Vec<String>
{
    package_infos.iter().filter(|&&pi| !pi.is_valid).map(|&pi| pi.package).collect()
}

fn do_the_thing(foos: &[PackageInfo]) -> &[String]{ 
    let bar0_foos = foos.iter().filter(|f| f.is_valid == true).map(|f| f.package).collect::<Vec<String>>();
    return &bar0_foos;
}

// pub fn get_invalid_packages(package_infos: Vec<PackageInfo>) -> Vec<String>
// {
//     package_infos.iter().filter(|&&pi| !pi.is_valid).map(|&pi| pi.package).collect()
// }

// pub fn get_invalid_packages(package_infos: Vec<PackageInfo>) -> Vec<String>
// {
//     package_infos.iter().filter(|&&pi| !pi.is_valid).map(|&pi| pi.package).collect()
// }

// pub fn get_invalid_packages(package_infos: Vec<PackageInfo>) -> Vec<String>
// {
//     package_infos.iter().filter(|&&pi| !pi.is_valid).map(|&pi| pi.package).collect()
// }

// pub fn get_invalid_packages(package_infos: Vec<PackageInfo>) -> Vec<String>
// {
//     package_infos.iter().filter(|&&pi| !pi.is_valid).map(|&pi| pi.package).collect()
// }

