use std::io;

pub fn print_err(err: io::Error) {
    eprint!("\x1B[91m");
    eprintln!("Error: {err}");
    eprint!("\x1B[0m");
}

pub fn print_err_str(err: &str) {
    eprint!("\x1B[91m");
    eprintln!("{err}");
    eprint!("\x1B[0m");
}
