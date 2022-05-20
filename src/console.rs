use crate::errors;
use std::io::{stdout, Write};

pub fn print(s: &str) {
    print!("{s}");
    stdout().flush().expect(errors::UNRECOVERABLE);
}

pub fn print_line(s: &str) {
    println!("{s}");
    stdout().flush().expect(errors::UNRECOVERABLE);
}
