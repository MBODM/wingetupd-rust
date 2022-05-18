# wingetupd-rust
A tiny command line tool, using [WinGet](https://docs.microsoft.com/en-us/windows/package-manager/winget) to update a user-defined set of packages on a Windows machine.

### Reasons
I recently was curious about the Rust programming language. So i started a redevelopment of my existing [wingetupd](https://github.com/MBODM/wingetupd) project. The `wingetupd-rust.exe` shall become exactly the same tool as the `wingetupd.exe`, just developed in Rust.

Another reason was also: `wingetupd.exe` (as a .NET 6 self-contained application) is around 10-15 MB in size. The `wingetupd-rust.exe` will become more like 0,5-1 MB in size.

But the main reason was: Let´s have some fun with Rust! 😁

#### So let´s get rusty.



Note to myself:

`fn main() {
    let a = "hallo v1.0.1 und grüsse v2.0.2 zumir-06-17";
    let cap: Vec<&str> = Regex::new(r"\d+(\.\d+)+").unwrap().find_iter(a).map(|x| x.as_str()).collect();
    println!("{}", cap.len());
    let s1 = cap[0];
    println!("{}", cap[0]);
    println!("{}", cap[1]);
}

use std::io::BufReader; 
use std::io::BufRead; 
use std::io; 
use std::fs; 
 
fn main() -> io::Result<()> { 
    let mut args = std::env::args(); 
    args.next(); 
    for arg in args { 
        let lines = file_to_vec(arg)?; 
        println!("{:?}", lines); 
    } 
 
    Ok(()) 
} 
 
fn file_to_vec(filename: String) -> io::Result<Vec<String>> { 
    let file_in = fs::File::open(filename)?; 
    let file_reader = BufReader::new(file_in); 
    Ok(file_reader.lines().filter_map(io::Result::ok).collect()) 
}`

