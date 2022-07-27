use super::parse::WinGetParseResult;

#[derive(Debug)]
pub struct WinGetAnalyzeResult {
    package_has_update: bool,
}

pub fn analyze_list_output(list_output: &str, parse_result: &WinGetParseResult) -> WinGetAnalyzeResult {
    let has_upd_text = list_output.contains(" Verfügbar ") || list_output.contains(" Available ");
    let has_new_version = !parse_result.new_version.is_empty();
    let package_has_update = has_upd_text && has_new_version;
    WinGetAnalyzeResult {
        package_has_update,
    }
}
