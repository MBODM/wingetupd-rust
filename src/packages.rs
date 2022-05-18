use crate::parser;
use crate::winget;

pub fn search(package_id: &str) -> Result<bool, String> {
    let package_id = package_id.trim();
    assert!(!package_id.is_empty());
    let command = format!("search --exact --id {}", package_id);
    let result = winget::run(command.as_str())?;
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

impl ListResult {
    pub const fn new() -> ListResult {
        return ListResult {
            package_id: String::new(),
            is_installed: false,
            is_updatable: false,
            installed_version: String::new(),
            update_version: String::new(),
        };
    }
}

pub fn list(package_id: &str) -> Result<ListResult, String> {
    let package_id = package_id.trim();
    assert!(!package_id.is_empty());
    let command = format!("list --exact --id {}", package_id);
    let result = winget::run(command.as_str())?;
    let list_result = ListResult::new();
    list_result.package_id = package_id.to_string();
    if result.exit_code == 0 && result.console_output.contains(package_id) {
        let parser_result = parser::parse_winget_list_output(result.console_output.as_str());
        list_result.is_installed  = false;
        list_result.is_updatable  =false;
        return Ok(list_result);
    } else {
        return Ok(list_result);
    }
}

// pub fn upgrade(id: &str) -> Result<bool, Box<dyn Error>> {
//     let result = winget::run()?;
//     return Ok(result.exit_code == 0);
// }
