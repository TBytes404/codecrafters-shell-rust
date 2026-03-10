#[allow(unused_imports)]
use std::io::{self, Write};
use std::ops::Deref;

const COMMAND_PROMPT: &str = "$ ";

fn main() {
    loop {
        show_prompt(COMMAND_PROMPT);
        let (cmd, args) = read_command();
        match cmd.deref() {
            "exit" => break,
            "echo" => println!("{}", args),
            "type" => {
                let arg = match args.split_once(" ") {
                    Some((arg, _)) => arg,
                    _ => args.as_str(),
                };
                if ["exit", "echo", "type"].contains(&arg) {
                    println!("{} is a builtin", arg)
                } else {
                    println!("{}: not found", arg)
                }
            }
            _ => println!("{}: command not found", cmd),
        }
    }
}

fn show_prompt(sym: &str) {
    print!("{}", sym);
    io::stdout().flush().unwrap();
}

fn read_command() -> (String, String) {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let line = line.trim();

    match line.split_once(" ") {
        Some((cmd, args)) => (cmd.into(), args.into()),
        None => (line.into(), "".into()),
    }
}
