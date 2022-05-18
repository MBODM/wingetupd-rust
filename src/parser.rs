extern crate regex;
use regex::Regex;

const ERROR_NO_VERSION_AT_ALL: &str = "WinGet list output doesn´t contain any version numbers.";
const ERROR_TOO_MANY_VERSIONS: &str = "WinGet list ouput contains more than 2 version numbers.";
const ERROR_MATCH_VERSION1: &str = "Can not get 1st regex match from regex matches.";
const ERROR_MATCH_VERSION2: &str = "Can not get 2nd regex match from regex matches.";

#[derive(Debug)]
pub struct ParseResult {
    pub is_updatable: bool,
    pub old_version: String,
    pub new_version: String,
}

pub fn parse_winget_list_output(list_output: &str) -> Result<ParseResult, String> {
    let list_output = list_output.trim();
    assert!(!list_output.is_empty());
    let regex = Regex::new(r"\d+(\.\d+)+").map_err(|err| err.to_string())?;
    let matches = regex.captures_iter(list_output);
    let count = matches.count();
    if count < 1 {
        return Err(ERROR_NO_VERSION_AT_ALL.to_string());
    }
    // if matches.count() > 2 {
    //     return Err(ERROR_TOO_MANY_VERSIONS.to_string());
    // }

    let v = Vec::new();
    v.push("");


    for mat in regex.find_iter(list_output) {
        println!("{:?}", mat);
        v.push(mat.as_str());
    }

    //let m = matches.nth(0).unwrap();
    let s = "".to_string();

    let my_match_option = matches.nth(0);
    let my_match = my_match_option.unwrap();
    print!("{}", v[0]);
    // let new_version = match matches.nth(1)
    // {
    //     Some(m) => m.as_str(),
    //     None => "",
    // };
    let has_text = list_output.contains(" Verfügbar ") || list_output.contains("Available");
    //let is_updatable = has_text && !new_version.is_empty();
    return Ok(ParseResult {
        is_updatable: false,
        old_version: "".to_string(),
        new_version: "".to_string(),
    });
}
