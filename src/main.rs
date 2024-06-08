use std::io::{self, Write};
use std::process::{self, exit, Command};
use std::{env, fs};
#[allow(unused_imports)]

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();

        stdin.read_line(&mut input).unwrap();

        let command = input.trim();
        let token = tokenize(command);

        match token[..] {
            ["exit", code] => process::exit(code.parse::<i32>().unwrap()),
            ["echo", ..] => println!("{}", token[1..].join(" ")),
            ["type", cmd] => type_command(cmd),
            [cmd, ..] => {
                execute_command(cmd, token[1..].join(" ").as_str());
            }
            [] => continue,
        }
    }
}

fn not_found(command: &str) {
    println!("{}: command not found", command);
}

fn tokenize(input: &str) -> Vec<&str> {
    input.split(' ').collect()
}

fn command_exist(cmd: &str) -> Result<String, bool> {
    let path_env = env::var("PATH").unwrap();
    let split = &mut path_env.split(':');

    match split.find(|path| fs::metadata(format!("{}/{}", path, cmd)).is_ok()) {
        Some(path) => Ok(path.to_string()),
        None => Err(false),
    }
}

fn type_command(cmd: &str) {
    if cmd == "exit" || cmd == "echo" || cmd == "type" {
        println!("{} is a shell builtin", cmd);
        return;
    }

    let path_env = std::env::var("PATH").unwrap();
    let split = &mut path_env.split(':');

    if let Some(path) = split.find(|path| std::fs::metadata(format!("{}/{}", path, cmd)).is_ok()) {
        println!("{cmd} is {path}/{cmd}");
    } else {
        println!("{cmd} not found");
    }
}

fn execute_command(cmd: &str, args: &str) {
    match command_exist(cmd) {
        Ok(path) => {
            let mut child = Command::new(path)
                .arg(args)
                .spawn()
                .expect("Failed to execute command");

            let status = child.wait().expect("Failed to wait on child");

            if !status.success() {
                exit(1);
            }
        }
        Err(_) => not_found(cmd),
    }
}
