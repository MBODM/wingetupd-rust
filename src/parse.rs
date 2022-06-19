extern crate regex;
use crate::errors::UNRECOVERABLE;
use regex::Regex;

#[derive(Debug)]
pub struct ParseResult {
    pub old_version: String,
    pub new_version: String,
}

pub fn parse_winget_listoutput(listoutput: &str) -> Result<ParseResult, String> {
    let list_output = listoutput.trim();
    assert!(!list_output.is_empty());
    let versions = get_versions(list_output);
    if versions.len() < 1 {
        return Err("WinGet list output not contains any version numbers.".to_string());
    }
    if versions.len() > 2 {
        return Err("WinGet list output contains more than 2 version numbers.".to_string());
    }
    let (old_version, new_version) = get_version_strings(&versions);
    print!("{:?}", versions);
    Ok(ParseResult {
        old_version,
        new_version,
    })
}

fn get_versions(output: &str) -> Vec<&str> {
    let regex = Regex::new(r"\d+(\.\d+)+").expect(UNRECOVERABLE);
    let versions = regex
        .find_iter(output)
        .map(|m| m.as_str())
        .collect::<Vec<_>>();
    versions
}

fn get_version_strings(versions: &Vec<&str>) -> (String, String) {
    let old_version = if versions.len() > 0 { versions[0] } else { "" };
    let new_version = if versions.len() > 1 { versions[1] } else { "" };
    (old_version.to_string(), new_version.to_string())
}
