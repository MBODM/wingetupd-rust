use super::{err::AppError, winget};

pub fn search(package: &str) -> Result<bool, AppError> {
    let package = package.trim();
    assert!(!package.is_empty());
    let params = format!("search --exact --id {package}");
    let data = winget::execute(&params).map_err(|err| AppError::new(err.msg))?;
    let valid = data.exit_code == 0 && data.console_output.contains(package);
    Ok(valid)
}

#[derive(Debug, Clone)]
pub struct ListData {
    pub package: String,
    pub is_installed: bool,
    pub is_updatable: bool,
    pub installed_version: String,
    pub update_version: String,
}

pub fn list(package: &str) -> Result<ListData, AppError> {
    let package = package.trim();
    assert!(!package.is_empty());
    let params = format!("list --exact --id {package}");
    let data = winget::execute(&params).map_err(|err| AppError::new(err.msg))?;
    let is_installed = data.exit_code == 0 && data.console_output.contains(package);
    if is_installed {
        let parse_data = winget::parse_list_output(&data.console_output)
            .map_err(|err| AppError::new(err.msg))?;
        Ok(build_list_data(package, Some(parse_data)))
    } else {
        Ok(build_list_data(package, None))
    }
}

pub fn upgrade(package: &str) -> Result<bool, AppError> {
    let package = package.trim();
    assert!(!package.is_empty());
    let params = format!("upgrade --exact --id {package}");
    let data = winget::execute(&params).map_err(|err| AppError::new(err.msg))?;
    let upgraded = data.exit_code == 0;
    Ok(upgraded)
}

fn build_list_data(package: &str, option: Option<winget::ParseListOutputData>) -> ListData {
    match option {
        Some(parse_data) => ListData {
            package: package.to_string(),
            is_installed: true,
            is_updatable: parse_data.has_upgrade,
            installed_version: parse_data.old_version,
            update_version: parse_data.new_version,
        },
        None => ListData {
            package: package.to_string(),
            is_installed: false,
            is_updatable: false,
            installed_version: "".to_string(),
            update_version: "".to_string(),
        },
    }
}
