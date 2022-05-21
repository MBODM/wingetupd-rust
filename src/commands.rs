use crate::app::AppResult;
use crate::parser;
use crate::winget;

pub fn search(package: &str) -> AppResult<bool> {
    let package = package.trim();
    assert!(!package.is_empty());
    let command = format!("search --exact --id {package}");
    let result = winget::execute(&command)?;
    let valid = result.exit_code == 0 && result.console_output.contains(package);
    Ok(valid)
}

#[derive(Debug)]
pub struct ListResult {
    pub package: String,
    pub is_installed: bool,
    pub is_updatable: bool,
    pub installed_version: String,
    pub update_version: String,
}

pub fn list(package: &str) -> AppResult<ListResult> {
    let package = package.trim();
    assert!(!package.is_empty());
    let command = format!("list --exact --id {package}");
    let result = winget::execute(&command)?;
    let installed = result.exit_code == 0 && result.console_output.contains(package);
    if installed {
        let parse_result = parser::parse_list_output(&result.console_output)?;
        Ok(build_list_result(package, true, Some(parse_result)))
    } else {
        Ok(build_list_result(package, false, None))
    }
}

pub fn upgrade(package: &str) -> AppResult<bool> {
    let package = package.trim();
    assert!(!package.is_empty());
    let command = format!("upgrade --exact --id {package}");
    let result = winget::execute(&command)?;
    let updated = result.exit_code == 0;
    Ok(updated)
}

fn build_list_result(
    package: &str,
    is_installed: bool,
    parse_result_option: Option<parser::ParseResult>,
) -> ListResult {
    let package = package.to_string();
    match parse_result_option {
        Some(parse_result) => ListResult {
            package,
            is_installed,
            is_updatable: parse_result.has_update,
            installed_version: parse_result.old_version,
            update_version: parse_result.new_version,
        },
        None => ListResult {
            package,
            is_installed,
            is_updatable: false,
            installed_version: "".to_string(),
            update_version: "".to_string(),
        },
    }
}
