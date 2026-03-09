#[allow(unused_imports)]
use std::io::{self, Write};

const COMMAND_PROMPT: &str = "$ ";

fn main() {
    loop {
        let cmd = wait_for_command();
        println!("{}: command not found", cmd)
    }
}

fn wait_for_command() -> String {
    show_prompt(COMMAND_PROMPT);
    read_command()
}

fn show_prompt(sym: &str) {
    print!("{}", sym);
    io::stdout().flush().unwrap();
}

fn read_command() -> String {
    let mut cmd = String::new();
    io::stdin().read_line(&mut cmd).unwrap();
    cmd.trim().into()
}
