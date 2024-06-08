#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;
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
            _ => not_found(command),
        }
    }
}
