use std::process::Command;
use std::{env, fs};

pub fn not_found(command: &str) {
    println!("{}: command not found", command);
}

fn command_exist(cmd: &str) -> bool {
    let path_env = env::var("PATH").unwrap();
    let paths = path_env.split(':');

    for path in paths {
        if fs::metadata(format!("{}/{}", path, cmd)).is_ok() {
            return true;
        }
    }
    false
}

pub fn execute_path_commands(cmd: &str, args: &[&str]) {
    if command_exist(cmd) {
        let mut child = Command::new(cmd)
            .args(args)
            .spawn()
            .expect("Failed to execute command");

        let _status = child.wait().expect("Failed to wait on child");
    } else {
        not_found(cmd);
    }
}
