use std::env;
use std::path::Path;

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
