use crate::app::commands;

use super::{common::AppError, commands::ListResult};

#[derive(Debug)]
pub struct PackageInfo {
    pub package: String,
    pub is_valid: bool,
    pub is_installed: bool,
    pub is_updatable: bool,
    pub installed_version: String,
    pub update_version: String,
}

pub fn analyze<T>(packages: Vec<String>, progress: T) -> Result<Vec<PackageInfo>, AppError>
where
    T: Fn() -> (),
{
    packages
        .iter()
        .map(|package| {
            let valid = {
                let package = package.trim();
                assert!(!package.is_empty());
                let params = format!("search --exact --id {package}");
                let valid = commands::search(&params)?;
                Ok(valid)
            }?;
            progress();
            let list_result = commands::list(package)?;
            progress();
            let package_info = build_package_info(package, valid, list_result);
            Ok(package_info)
        })
        .collect()
}

fn build_package_info(package: &String, is_valid: bool, list_result: ListResult) -> PackageInfo {
    PackageInfo {
        package: package.to_string(),
        is_valid,
        is_installed: list_result.is_installed,
        is_updatable: list_result.is_updatable,
        installed_version: list_result.installed_version,
        update_version: list_result.update_version,
    }
}
