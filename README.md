# wingetupd-rust
A tiny command line tool, using [WinGet](https://docs.microsoft.com/en-us/windows/package-manager/winget) to update a user-defined set of packages on a Windows machine.

### Reasons
I recently was curious about the Rust programming language. So i started a redevelopment of my existing [wingetupd](https://github.com/MBODM/wingetupd) project. The `wingetupd-rust.exe` shall be exactly the same tool as the `wingetupd.exe`, just built with Rust.

A side reason was: `wingetupd.exe` (as a .NET 6 self-contained application) is around 10-15 MB in size. The `wingetupd-rust.exe` will become more like 0,5-1 MB in size.

But the main reason is: Have fun with Rust! 😁
