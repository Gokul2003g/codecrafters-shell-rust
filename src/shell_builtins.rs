use std::env;
use std::path::Path;

pub fn change_directory(args: &str) {
    let path = Path::new(args);
    if env::set_current_dir(path).is_err() {
        // cd: /does_not_exist: No such file or directory
        println!("cd: {}: No such file or directory", path.display());
    }
}
