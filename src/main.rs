use std::io::{self, Write};
use std::process::exit;

mod commands;
mod tokenize;

use commands::{execute_path_commands, type_command};
use tokenize::tokenize;

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

        match token.as_slice() {
            ["exit", code] => exit(code.parse::<i32>().unwrap_or(0)),
            ["echo", ..] => println!("{}", token[1..].join(" ")),
            ["type", cmd] => type_command(cmd),
            [cmd, ..] => {
                execute_path_commands(cmd, &token[1..]);
            }
            [] => continue,
        }
    }
}
