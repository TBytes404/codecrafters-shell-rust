#[allow(unused_imports)]
use std::io::{self, Write};
use std::{
    env::{current_dir, set_current_dir, split_paths, var_os},
    path::{Path, PathBuf},
    process::Command,
};

use is_executable::IsExecutable;

const COMMAND_PROMPT: &str = "$ ";
const BUILTIN_COMMANDS: [&str; 5] = ["exit", "echo", "type", "pwd", "cd"];

fn main() {
    loop {
        show_prompt(COMMAND_PROMPT);
        let input = read_command();
        let (cmd, args) = first_arg(&input);

        match cmd {
            "exit" => break,
            "echo" => println!("{}", args),
            "type" => builtin_type(args),
            "pwd" => println!("{}", current_dir().unwrap().display()),
            "cd" => builtin_cd(args),
            exe => run_executable(exe, args),
        }
    }
}

fn run_executable(cmd: &str, args: &str) {
    if find_executable(cmd).is_some() {
        let output = Command::new(cmd).args(args.split(" ")).output().unwrap();
        print!("{}", String::from_utf8_lossy(&output.stdout))
    } else {
        println!("{}: command not found", cmd)
    }
}

fn builtin_cd(args: &str) {
    let (arg, _) = first_arg(args);
    let path = if arg == "~" {
        PathBuf::from(var_os("HOME").unwrap())
    } else {
        PathBuf::from(arg)
    };
    if path.is_dir() {
        set_current_dir(path).unwrap();
    } else {
        println!("cd: {}: No such file or directory", arg)
    }
}

fn builtin_type(args: &str) {
    let (arg, _) = first_arg(args);
    if BUILTIN_COMMANDS.contains(&arg) {
        println!("{} is a shell builtin", arg)
    } else if let Some(path) = find_executable(arg) {
        println!("{} is {}", arg, path.display())
    } else {
        println!("{}: not found", arg)
    }
}

fn find_executable(arg: &str) -> Option<PathBuf> {
    let paths = { var_os("PATH").unwrap() };
    split_paths(&paths)
        .map(|dir| dir.join(arg))
        .find(|p| p.is_executable())
}

fn show_prompt(sym: &str) {
    print!("{}", sym);
    io::stdout().flush().unwrap();
}

fn read_command() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn first_arg(args: &str) -> (&str, &str) {
    match args.split_once(|c: char| c.is_whitespace()) {
        Some((f, e)) => (f, e.trim_start()),
        None => (args, ""),
    }
}
