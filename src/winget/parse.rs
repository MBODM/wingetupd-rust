extern crate regex;
use regex::Regex;

#[derive(Debug)]
pub struct WinGetParseResult {
    pub old_version: String,
    pub new_version: String,
}

pub fn parse_list_output(list_output: &str) -> Result<WinGetParseResult, String> {
    let list_output = list_output.trim();
    assert!(!list_output.is_empty());
    let regex = Regex::new(r"\d+(\.\d+)+").map_err(|_| "WinGet output regex parsing failed.".to_string())?;
    let versions = regex
        .find_iter(list_output)
        .map(|m| m.as_str())
        .collect::<Vec<_>>();
    if versions.len() < 1 {
        return Err("WinGet list output not contains any version numbers.".to_string());
    }
    if versions.len() > 2 {
        return Err("WinGet list output contains more than 2 version numbers.".to_string());
    }
    let old_version = (if versions.len() > 0 { versions[0] } else { "" }).to_string();
    let new_version = (if versions.len() > 1 { versions[1] } else { "" }).to_string();
    Ok(WinGetParseResult {
        old_version,
        new_version,
    })
}
