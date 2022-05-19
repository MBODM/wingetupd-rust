use crate::commands;

#[derive(Debug)]
pub struct PackageInfo {
    pub package_id: String,
    pub is_valid: bool,
    pub is_installed: bool,
    pub is_updatable: bool,
    pub installed_version: String,
    pub update_version: String,
}

pub fn analyze(packages: Vec<String>, progress: fn()) -> Result<Vec<PackageInfo>, String> {
    let package_infos: Result<Vec<PackageInfo>, String> = packages
        .iter()
        .map(|package| {
            let package_id: String = package.into();
            let valid_package = commands::search(&package_id)?;
            progress();
            let list_result = commands::list(&package_id)?;
            progress();
            let package_info = create_package_info(package_id, valid_package, list_result);
            return Ok(package_info);
        })
        .collect();
    return package_infos;
}

fn create_package_info(
    package_id: String,
    is_valid: bool,
    list_result: commands::ListResult,
) -> PackageInfo {
    return PackageInfo {
        package_id,
        is_valid,
        is_installed: list_result.is_installed,
        is_updatable: list_result.is_updatable,
        installed_version: list_result.installed_version,
        update_version: list_result.update_version,
    };
}
