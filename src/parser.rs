extern crate regex;
use regex::Regex;

const ERROR_NO_VERSION_AT_ALL: &str = "WinGet list output doesn´t contain any version numbers.";
const ERROR_TOO_MANY_VERSIONS: &str = "WinGet list ouput contains more than 2 version numbers.";

#[derive(Debug)]
pub struct ParseResult {
    pub old_version: String,
    pub new_version: String,
    pub has_update: bool,
}

pub fn parse_winget_list_output(list_output: &str) -> Result<ParseResult, String> {
    let list_output = list_output.trim();
    assert!(!list_output.is_empty());
    let versions = get_versions(list_output)?;
    if versions.len() < 1 {
        return Err(ERROR_NO_VERSION_AT_ALL.to_string());
    }
    if versions.len() > 2 {
        return Err(ERROR_TOO_MANY_VERSIONS.to_string());
    }
    let (old_version, new_version) = get_strings(versions);
    let has_update = has_update(list_output, &new_version);
    return Ok(ParseResult {
        old_version,
        new_version,
        has_update,
    });
}

fn get_versions(list_output: &str) -> Result<Vec<&str>, String> {
    let regex = Regex::new(r"\d+(\.\d+)+").map_err(|err| err.to_string())?;
    let versions: Vec<&str> = regex.find_iter(list_output).map(|m| m.as_str()).collect();
    return Ok(versions);
}

fn get_strings(versions: Vec<&str>) -> (String, String) {
    let old_version = (if versions.len() > 0 { versions[0] } else { "" }).to_string();
    let new_version = (if versions.len() > 1 { versions[1] } else { "" }).to_string();
    return (old_version, new_version);
}

fn has_update(list_output: &str, new_version: &str) -> bool {
    let has_upd_text = list_output.contains(" Verfügbar ") || list_output.contains(" Available ");
    let has_new_version = !new_version.is_empty();
    return has_upd_text && has_new_version;
}
