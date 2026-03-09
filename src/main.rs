#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    print!("$ ");
    // io::stdout().flush().unwrap();
    let cmd = read_command();
    println!("{}: command not found", cmd)
}

fn read_command() -> String {
    let mut cmd = String::new();
    io::stdin().read_line(&mut cmd).unwrap();
    cmd.trim().into()
}
