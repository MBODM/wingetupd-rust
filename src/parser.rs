extern crate regex;
use regex::Regex;

pub struct ParseResult
{
    pub is_updatable: bool,
    pub old_version: String,
    pub new_version: String,
    pub has_err: bool,
    pub err_msg: String,
}

impl ParseResult {
    pub const fn new() -> ParseResult {
        return ParseResult{
            is_updatable: false,
            old_version: String::new(),
            new_version: String::new(),
            has_err: false,
            err_msg: String::new(),
        }
    }

    pub const fn from(is_updatable: bool, old_version: String, new_version: String, has_err: bool, err_msg: String) -> ParseResult {
        return ParseResult{
            is_updatable,
            old_version,
            new_version,
            has_err,
            err_msg,
        }
    }
}

// pub fn Parse(console_output: &str) -> ParseResult{
//     // WinGet list output example:
//     /*
//     Name                         ID                                    Version Verfügbar Quelle
//     -------------------------------------------------------------------------------------------
//     Visual Studio Community 2022 Microsoft.VisualStudio.2022.Community 17.1.6  17.2.0    winget
//     */

//     let regex = Regex::new(r"\d+(\.\d+)+").expect("parser");
//     let matches = regex.find_iter("").count();

//     if matches == 0
//     {
//         return ParseResult::new();
//     }

//     if matches > 2
//     {
//         //error
//     }

            // var regex = new Regex(@"\d+(\.\d+)+");
            // var matches = regex.Matches(listOutput);

            // if (!matches.Any())
            // {
            //     var message = "Given argument is not a valid WinGet list output, since it doesn´t contain any version numbers.";
            //     throw new ArgumentException(message, nameof(listOutput));
            // }

            // if (matches.Count > 2)
            // {
            //     var message = "Given argument is not a valid WinGet list output, since it contains more than 2 version numbers.";
            //     throw new ArgumentException(message, nameof(listOutput));
            // }

            // var oldVersion = matches.First().Value;
            // var newVersion = matches.Count == 2 ? matches.Last().Value : string.Empty;
            // var isUpdatable = (listOutput.Contains(" Verfügbar ") || listOutput.Contains(" Available ")) && !string.IsNullOrEmpty(newVersion);

            // return new WinGetOutputParserListResult(isUpdatable, oldVersion, newVersion);

//}
