use crate::parser;
use crate::winget;

pub fn search(package_id: &str) -> Result<bool, String> {
    let package_id = package_id.trim();
    assert!(!package_id.is_empty());
    let command = format!("search --exact --id {}", package_id);
    let result = winget::run(&command)?;
    let valid = result.exit_code == 0 && result.console_output.contains(package_id);
    return Ok(valid);
}

#[derive(Debug)]
pub struct ListResult {
    pub package_id: String,
    pub is_installed: bool,
    pub is_updatable: bool,
    pub installed_version: String,
    pub update_version: String,
}

pub fn list(package_id: &str) -> Result<ListResult, String> {
    let package_id = package_id.trim();
    assert!(!package_id.is_empty());
    let command = format!("list --exact --id {}", package_id);
    let run_result = winget::run(&command)?;
    let installed = run_result.exit_code == 0 && run_result.console_output.contains(package_id);
    if installed {
        let parser_result = parser::parse_winget_list_output(&run_result.console_output)?;
        return Ok(build_list_result(package_id, true, Some(parser_result)));
    } else {
        return Ok(build_list_result(package_id, false, None));
    }
}

pub fn upgrade(package_id: &str) -> Result<bool, String> {
    let package_id = package_id.trim();
    assert!(!package_id.is_empty());
    let command = format!("upgrade --exact --id {}", package_id);
    let run_result = winget::run(&command)?;
    let updated = run_result.exit_code == 0;
    return Ok(updated);
}

fn build_list_result(
    package_id: &str,
    is_installed: bool,
    parse_result_option: Option<parser::ParseResult>,
) -> ListResult {
    let package_id = package_id.to_string();
    return match parse_result_option {
        Some(parse_result) => ListResult {
            package_id,
            is_installed,
            is_updatable: parse_result.has_update,
            installed_version: parse_result.old_version,
            update_version: parse_result.new_version,
        },
        None => ListResult {
            package_id,
            is_installed,
            is_updatable: false,
            installed_version: String::from(""),
            update_version: String::from(""),
        },
    };
}
