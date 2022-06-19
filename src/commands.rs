use crate::app::AppResult;
use crate::parse;
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
        let parse_result = parse::parse_winget_listoutput(&result.console_output)?;
        let has_update = has_update(&result.console_output, &parse_result.new_version);
        Ok(build_list_result(package, true, has_update, Some(parse_result)))
    } else {
        Ok(build_list_result(package, false, false, None))
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
    has_update: bool,
    parse_result_option: Option<parse::ParseResult>,
) -> ListResult {
    let package = package.to_string();
    match parse_result_option {
        Some(parse_result) => ListResult {
            package,
            is_installed,
            is_updatable: has_update,
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

fn has_update(output: &str, new_version: &str) -> bool {
    let has_upd_text = output.contains(" Verfügbar ") || output.contains(" Available ");
    let has_new_version = !new_version.is_empty();
    has_upd_text && has_new_version
}
