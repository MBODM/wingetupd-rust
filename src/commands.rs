use crate::parser;
use crate::winget;

pub fn search(package: &str) -> Result<bool, String> {
    let package = package.trim();
    assert!(!package.is_empty());
    let command = format!("search --exact --id {package}");
    let result = winget::execute(&command)?;
    let valid = result.exit_code == 0 && result.console_output.contains(package);
    return Ok(valid);
}

#[derive(Debug)]
pub struct ListResult {
    pub package: String,
    pub is_installed: bool,
    pub is_updatable: bool,
    pub installed_version: String,
    pub update_version: String,
}

pub fn list(package: &str) -> Result<ListResult, String> {
    let package = package.trim();
    assert!(!package.is_empty());
    let command = format!("list --exact --id {package}");
    let result = winget::execute(&command)?;
    let installed = result.exit_code == 0 && result.console_output.contains(package);
    if installed {
        let parse_result = parser::parse_winget_list_output(&result.console_output)?;
        return Ok(build_list_result(package, true, Some(parse_result)));
    } else {
        return Ok(build_list_result(package, false, None));
    }
}

pub fn upgrade(package: &str) -> Result<bool, String> {
    let package = package.trim();
    assert!(!package.is_empty());
    let command = format!("upgrade --exact --id {package}");
    let result = winget::execute(&command)?;
    let updated = result.exit_code == 0;
    return Ok(updated);
}

fn build_list_result(
    package: &str,
    is_installed: bool,
    parse_result_option: Option<parser::ParseResult>,
) -> ListResult {
    let package = package.to_string();
    return match parse_result_option {
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
            installed_version: String::from(""),
            update_version: String::from(""),
        },
    };
}
