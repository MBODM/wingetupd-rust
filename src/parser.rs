extern crate regex;
use crate::errors;
use regex::Regex;

#[derive(Debug)]
pub struct ParseResult {
    pub old_version: String,
    pub new_version: String,
    pub has_update: bool,
}

pub fn parse_winget_list_output(list_output: &str) -> Result<ParseResult, String> {
    let list_output = list_output.trim();
    assert!(!list_output.is_empty());
    let versions = get_versions(list_output);
    if versions.len() < 1 {
        return Err(String::from(
            "WinGet list output not contains any version numbers.",
        ));
    }
    if versions.len() > 2 {
        return Err(String::from(
            "WinGet list output contains more than 2 version numbers.",
        ));
    }
    let (old_version, new_version) = get_strings(versions);
    let has_update = has_update(list_output, &new_version);
    return Ok(ParseResult {
        old_version,
        new_version,
        has_update,
    });
}

fn get_versions(list_output: &str) -> Vec<&str> {
    let regex = Regex::new(r"\d+(\.\d+)+").expect(errors::UNRECOVERABLE);
    let versions: Vec<&str> = regex.find_iter(list_output).map(|m| m.as_str()).collect();
    return versions;
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
