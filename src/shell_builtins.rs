use std::path::Path;
use std::{env, fs};

pub fn change_directory(args: &str) {
    let path = if args == "~" {
        dirs::home_dir().expect("Error getting home directory")
    } else {
        Path::new(args).to_path_buf()
    };

    if env::set_current_dir(&path).is_err() {
        println!("cd: {}: No such file or directory", path.display());
    }
}

pub fn type_command(cmd: &str) {
    if cmd == "exit" || cmd == "echo" || cmd == "type" || cmd == "cd" {
        println!("{} is a shell builtin", cmd);
        return;
    }

    let path_env = std::env::var("PATH").unwrap();
    let paths = path_env.split(':');

    for path in paths {
        if fs::metadata(format!("{}/{}", path, cmd)).is_ok() {
            println!("{cmd} is {}/{}", path, cmd);
            return;
        }
    }
    println!("{cmd} not found");
}
