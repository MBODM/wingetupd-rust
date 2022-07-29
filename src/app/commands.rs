use super::common::AppError;
use crate::winget;

pub fn search(package: &str) -> Result<bool, AppError> {
    let package = package.trim();
    assert!(!package.is_empty());
    let params = format!("search --exact --id {package}");
    let result = winget::execute(&params).map_err(|err| AppError::new(err.message))?;
    let valid = result.exit_code == 0 && result.console_output.contains(package);
    Ok(valid)
}

#[derive(Debug, Clone)]
pub struct ListResult {
    pub package: String,
    pub is_installed: bool,
    pub is_updatable: bool,
    pub installed_version: String,
    pub update_version: String,
}

pub fn list(package: &str) -> Result<ListResult, AppError> {
    let package = package.trim();
    assert!(!package.is_empty());
    let params = format!("list --exact --id {package}");
    let result = winget::execute(&params).map_err(|err| AppError::new(err.message))?;
    let package_installed = result.exit_code == 0 && result.console_output.contains(package);
    let list_result = build_list_result(package, package_installed);
    if package_installed {
        let parse_result = winget::parse_list_output(&result.console_output)
            .map_err(|err| AppError::new(err.message))?;
        list_result.installed_version = parse_result.old_version;
        list_result.update_version = parse_result.new_version;
        list_result.is_updatable = parse_result.has_upgrade;
    }
    Ok(list_result)
}

pub fn upgrade(package: &str) -> Result<bool, AppError> {
    let package = package.trim();
    assert!(!package.is_empty());
    let params = format!("upgrade --exact --id {package}");
    let result = winget::execute(&params).map_err(|err| AppError::new(err.message))?;
    let updated = result.exit_code == 0;
    Ok(updated)
}

fn build_list_result(package: &str, is_installed: bool) -> ListResult {
    ListResult {
        package: package.to_string(),
        is_installed,
        is_updatable: false,
        installed_version: "".to_string(),
        update_version: "".to_string(),
    }
}
