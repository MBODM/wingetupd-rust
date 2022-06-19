// This module is rather short now, because it´s not possible to
// create a fixed pattern that´s matching the "ugly head" of the
// WinGet output string always and for 100% sure. This statement
// is true for the single progress bar chars (see older versions
// of the code in GitHub´s commit history), as well as for these
// weird download graphs (see some example graphs in the comment
// below). Therefore it seems the most secure way is to find the
// position of the always-existing "Name " substring and to grab
// the "tail" of the WinGet output string from there. Imo that´s
// fine, because if WinGet ever changing things that drastically,
// chances are good this tool will need different changes anyway.

pub fn prettify_winget_output(winget_output: &str) -> &str {
    // Solely prettify output containing the "Name " substring, like
    // in example some successful WinGet "search" or "list" commands.
    // Be careful when using this function for another WinGet output.
    // Remember: The find() function returns a byte position and not
    // a character position (see Rust docs). But this doesn´t matter
    // here. What matters is what´s done starting FROM that position.
    let pos_option = winget_output.find("Name ");
    let str_result = match pos_option {
        Some(i) => &winget_output[i..],
        None => winget_output,
    };
    str_result
}

// This is some example of above mentioned download graphs:
//
// ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒  1%\r ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒  2%\r ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ 3%\r
// █▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒  5%\r ██▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒  9%\r █████▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ 18%\r
// █████▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ 19%\r █████████▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ 30%\r █████████▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ 31%\r
// █████████▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ 32%\r █████████▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ 33%\r ██████████▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ 34%\r
// ██████████▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ 35%\r ██████████████▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ 48%\r ██████████████▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ 49%\r
// ███████████████▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ 50%\r ███████████████▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ 51%\r ███████████████▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ 52%\r
// ███████████████▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ 53%\r ████████████████▒▒▒▒▒▒▒▒▒▒▒▒▒▒ 54%\r ████████████████▒▒▒▒▒▒▒▒▒▒▒▒▒▒ 55%\r
// ████████████████▒▒▒▒▒▒▒▒▒▒▒▒▒▒ 56%\r █████████████████▒▒▒▒▒▒▒▒▒▒▒▒▒ 56%\r █████████████████▒▒▒▒▒▒▒▒▒▒▒▒▒ 57%\r
// █████████████████▒▒▒▒▒▒▒▒▒▒▒▒▒ 59%\r ██████████████████▒▒▒▒▒▒▒▒▒▒▒▒ 60%\r ██████████████████▒▒▒▒▒▒▒▒▒▒▒▒ 61%\r
// ██████████████████▒▒▒▒▒▒▒▒▒▒▒▒ 62%\r ██████████████████▒▒▒▒▒▒▒▒▒▒▒▒ 63%\r ███████████████████▒▒▒▒▒▒▒▒▒▒▒ 64%\r
// ███████████████████▒▒▒▒▒▒▒▒▒▒▒ 65%\r ███████████████████▒▒▒▒▒▒▒▒▒▒▒ 66%\r ████████████████████▒▒▒▒▒▒▒▒▒▒ 67%\r
