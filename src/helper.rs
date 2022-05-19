use std::io::{stdout, Write};

pub fn console_write(s: &str) {
    print!("{}", s);
    stdout().flush().unwrap();
}

pub fn console_write_line(s: &str) {
    println!("{}", s);
    stdout().flush().unwrap();
}
