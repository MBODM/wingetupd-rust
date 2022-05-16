mod winget;

pub fn search_package(id: &str) -> bool {
    let result = winget::winget_run(format!("search --exact --id {}", id))?;
    return result.exit_code == 0;
}

pub struct ListResult {
    pub package: String,
    pub is_installed: bool,
    pub is_updatable: bool,
    pub installed_version: String,
    pub update_version: String,
}

pub fn list_package(id: &str) -> ListResult {
    let result = winget::winget_run(format!("list --exact --id {}", id))?;
    return ListResult {
        package: id.to_string(),
        is_installed: false,
        is_updatable: false,
        installed_version: String::from(""),
        update_version: String::from(""),
    };
}

pub fn upgrade_package(id: &str) -> bool {
    let result = winget::winget_run(format!("search --exact --id {}", id))?;
    return result.exit_code == 0;
}
