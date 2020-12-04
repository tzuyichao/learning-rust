use std::process::Command;

fn main() {
    let output = Command::new("Write-Output")
        .arg("Hello world")
        .output()
        .expect("Failed to execute command");
    println!("{:?}", output);
}