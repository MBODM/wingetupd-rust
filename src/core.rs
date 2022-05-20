use crate::commands;

#[derive(Debug)]
pub struct PackageInfo {
    pub package: String,
    pub is_valid: bool,
    pub is_installed: bool,
    pub is_updatable: bool,
    pub installed_version: String,
    pub update_version: String,
}

pub fn analyze<T>(packages: Vec<String>, progress: T) -> Result<Vec<PackageInfo>, String>
where
    T: Fn() -> (),
{
    let package_infos: Result<Vec<PackageInfo>, String> = packages
        .iter()
        .map(|package| {
            let package: String = package.into();
            let valid = commands::search(&package)?;
            progress();
            let list_result = commands::list(&package)?;
            progress();
            let package_info = build_package_info(package, valid, list_result);
            return Ok(package_info);
        })
        .collect();
    return package_infos;
}

fn build_package_info(
    package: String,
    is_valid: bool,
    list_result: commands::ListResult,
) -> PackageInfo {
    return PackageInfo {
        package,
        is_valid,
        is_installed: list_result.is_installed,
        is_updatable: list_result.is_updatable,
        installed_version: list_result.installed_version,
        update_version: list_result.update_version,
    };
}
