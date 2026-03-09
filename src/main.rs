#[allow(unused_imports)]
use std::io::{self, Write};
use std::ops::Deref;

const COMMAND_PROMPT: &str = "$ ";

fn main() {
    loop {
        match read_command().deref() {
            "exit" => break,
            cmd => println!("{}: command not found", cmd),
        }
    }
}

fn show_prompt(sym: &str) {
    print!("{}", sym);
    io::stdout().flush().unwrap();
}

fn read_command() -> String {
    show_prompt(COMMAND_PROMPT);
    let mut cmd = String::new();
    io::stdin().read_line(&mut cmd).unwrap();
    cmd.trim().into()
}
