use std::io::{self, Write};
use std::process::{exit, Command};
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
            ["exit", code] => exit(code.parse::<i32>().unwrap()),
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

fn command_exist(cmd: &str) -> bool {
    let path_env = env::var("PATH").unwrap();
    let split = &mut path_env.split(':');

    match split.find(|path| fs::metadata(format!("{}/{}", path, cmd)).is_ok()) {
        Some(_path) => true,
        None => false,
    }
}

fn execute_command(cmd: &str, args: &str) {
    if command_exist(cmd) {
        let mut child = Command::new(cmd)
            .arg(args)
            .spawn()
            .expect("Failed to execute command");

        let _status = child.wait().expect("Failed to wait on child");
    } else {
        not_found(cmd);
    }
}
