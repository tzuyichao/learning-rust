use std::env;

fn main() {
    match env::current_exe() {
        Ok(exe_path) => println!("Path of this executable is: {}", exe_path.display()),
	Err(e) => println!("failed to get current exe path: {e}"),
    }
    let temp_dir = env::temp_dir();
    println!("Temporary directory: {}", temp_dir.display());
}
