use crate::winget;

pub fn search(package: &str) -> Result<bool, String> {
    let package = package.trim();
    assert!(!package.is_empty());
    let params = format!("search --exact --id {package}");
    let result = winget::execute(&params)?;
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

pub fn list(package: &str) -> Result<ListResult, String> {
    let package = package.trim();
    assert!(!package.is_empty());
    let params = format!("list --exact --id {package}");
    let result = winget::execute(&params)?;
    let package_installed = result.exit_code == 0 && result.console_output.contains(package);
    let list_result = build_list_result(package, package_installed);
    if package_installed {
        let parse_result = winget::parse(&result.console_output)?;
        let analyze_result = winget::analyze(&result.console_output, &parse_result);
        list_result.installed_version = parse_result.old_version;
        list_result.update_version = parse_result.new_version;
    }
    Ok(list_result)
}

pub fn upgrade(package: &str) -> Result<bool, String> {
    let package = package.trim();
    assert!(!package.is_empty());
    let params = format!("upgrade --exact --id {package}");
    let result = winget::execute(&params)?;
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
