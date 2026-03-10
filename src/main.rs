#[allow(unused_imports)]
use std::io::{self, Write};
use std::{
    env::{split_paths, var_os},
    fs::read_dir,
    ops::Deref,
    process::Command,
};

use is_executable::IsExecutable;

const COMMAND_PROMPT: &str = "$ ";
const BUILTIN_COMMANDS: [&str; 3] = ["exit", "echo", "type"];

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
                if BUILTIN_COMMANDS.contains(&arg) {
                    println!("{} is a shell builtin", arg)
                } else if let Some(path) = find_executable(arg) {
                    println!("{} is {}", arg, path)
                } else {
                    println!("{}: not found", arg)
                }
            }
            cmd => {
                if find_executable(cmd).is_some() {
                    let output = Command::new(cmd).args(args.split(" ")).output().unwrap();
                    println!("{}", String::from_utf8_lossy(&output.stdout))
                } else {
                    println!("{}: command not found", cmd)
                }
            }
        }
    }
}

fn find_executable(arg: &str) -> Option<String> {
    let paths = { var_os("PATH").unwrap() };
    for path in split_paths(&paths).filter(|p| p.exists()) {
        for entry in read_dir(path).unwrap() {
            let entry = entry.unwrap().path();
            let file = entry.file_name().unwrap();
            if file == arg && entry.is_executable() {
                return Some(entry.display().to_string());
            }
        }
    }
    None
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
