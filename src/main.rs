mod app;
mod winget;

// How to gracefully exit a rust app with an exit code? See here:
// https://stackoverflow.com/questions/30281235/how-to-cleanly-end-the-program-with-an-exit-code
// https://stackoverflow.com/questions/24245276/why-does-rust-not-have-a-return-value-in-the-main-function-and-how-to-return-a

fn main() -> Result<(), ()> {
    println!();
    app::show_prolog();
    println!();
    match app::run() {
        Ok(show_epilog) => {
            if show_epilog {
                app::show_epilog();
            }
            // Since Rust 1.26 this leads to platform-specific EXIT_SUCCESS:
            Ok(())
        }
        Err(err_msg) => {
            println!("Error: {err_msg}");
            // Since Rust 1.26 this leads to platform-specific EXIT_FAILURE:
            Err(()) 
        }
    }
}
