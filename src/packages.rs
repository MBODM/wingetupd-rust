use super::{commands, err::AppError};

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
        .map(|&package| {
            let valid = commands::search(package.as_str())?;
            progress();
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
            progress();
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
