// book example4 @ page 4-19

fn open_file(file_path: String) -> Result<String, std::io::Error> {
    let result = std::fs::read_to_string(file_path);
    result
}

fn main() {
    let text = open_file("data.txt".to_string()).unwrap();
    println!("{text:?}");
}
