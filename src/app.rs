// No need for some code here, to verify name and version from cargo.toml file,
// since cargo will show an error, if name or version contains an empty string.

pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const AUTHOR: &str = "MBODM";
pub const DATE: &str = "2022-07-22";

// I added some slice type alias here, used everywhere in this application.
// Because to me, as a Rust beginner, the syntactic style of slices adds a
// lot of confusion, between all the borrow stuff and slices. Because both
// make use of the &-char. Also i won´t become a Rust developer on a daily
// base, so i need to understand what´s going on here, in let´s say a year,
// meanwhile maybe not written a single line of Rust code. So, therefore i
// made my life a bit easier, by using this type alias, when using a slice.

pub type SliceOf<'a, T> = &'a [T];
