use crate::winget;
use crate::parser;

pub fn search(package_id: &str) -> bool {
    let command = format!("search --exact --id {}", package_id);
    let result = winget::run(command.as_str());
    return result.exit_code == 0 && result.console_output.contains(package_id);
}

pub struct ListResult {
    pub package: String,
    pub is_installed: bool,
    pub is_updatable: bool,
    pub installed_version: String,
    pub update_version: String,
}

pub fn list(package_id: &str) -> ListResult {
    let command = format!("list --exact --id {}", package_id);
    let result = winget::run(command.as_str());

    if result.exit_code == 0 && result.console_output.contains(package_id) {
        let parser_result = parser::Parse(result.console_output.as_str());

        if (parser_result.)

        return ListResult {
            package: package_id.to_string(),
            is_installed: true,
            is_updatable: parser_result.is_updatable,
            installed_version: parser_result.old_version,
            update_version: parser_result.new_version,
        };
    } else {
        return ListResult {
            package: package_id.to_string(),
            is_installed: false,
            is_updatable: false,
            installed_version: String::from("")"",
            update_version: String::from(""),
        };
    }
}


// pub fn upgrade(id: &str) -> Result<bool, Box<dyn Error>> {
//     let result = winget::run()?;
//     return Ok(result.exit_code == 0);
// }
