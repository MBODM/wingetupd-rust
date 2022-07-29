use super::WinGetError;

extern crate regex;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct ParseListOutputData {
    pub old_version: String,
    pub new_version: String,
    pub has_upgrade: bool,
}

pub fn parse_list_output(list_output: &str) -> Result<ParseListOutputData, WinGetError> {
    let list_output = list_output.trim();
    assert!(!list_output.is_empty());
    let regex = Regex::new(r"\d+(\.\d+)+")
        .map_err(|_| WinGetError::new_str("WinGet output regex parsing failed."))?;
    let versions = regex
        .find_iter(list_output)
        .map(|m| m.as_str())
        .collect::<Vec<_>>();
    if versions.len() < 1 {
        return Err(WinGetError::new_str(
            "WinGet list output not contains any version numbers.",
        ));
    }
    if versions.len() > 2 {
        return Err(WinGetError::new_str(
            "WinGet list output contains more than 2 version numbers.",
        ));
    }
    let old_version = (if versions.len() > 0 { versions[0] } else { "" }).to_string();
    let new_version = (if versions.len() > 1 { versions[1] } else { "" }).to_string();
    // No version comparison here and better let WinGet decide this,
    // since WinGet only says "Available" if it´s really an upgrade.
    let contains_av = list_output.contains(" Available ") || list_output.contains(" Verfügbar ");
    let has_upgrade = !new_version.is_empty() && contains_av;
    Ok(ParseListOutputData {
        old_version,
        new_version,
        has_upgrade,
    })
}
