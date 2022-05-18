use crate::parser;
use crate::parser::ParseResult;
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
    if run_result.exit_code == 0 && run_result.console_output.contains(package_id) {
        let parser_result = parser::parse_winget_list_output(&run_result.console_output)?;
        return Ok(create_list_result(package_id, true, Some(parser_result)));
    } else {
        return Ok(create_list_result(package_id, false, None));
    }
}

fn create_list_result(
    package_id: &str,
    is_installed: bool,
    parse_result: Option<ParseResult>,
) -> ListResult {
    return ListResult {
        package_id: package_id.to_string(),
        is_installed,
        is_updatable: match parse_result { Some(val) => val.has_update, None => false },
        installed_version: if let Some(b) = parse_result { b.old_version } else { "".to_string() },
        update_version:if let Some(c) = parse_result { c.new_version} else { "".to_string() },
    };
}

// pub fn upgrade(id: &str) -> Result<bool, Box<dyn Error>> {
//     let result = winget::run()?;
//     return Ok(result.exit_code == 0);
// }
