use std::io::{BufReader, BufRead};

fn main() {
    let path = std::env::args().nth(1).expect("please provide file name");
    let n: i32;
    if std::env::args().len() <= 2 {
        n = 5;
    } else {
        n = std::env::args().nth(2).expect("please provide display rows number")
                .trim().parse().expect("number required");
    }

    if !std::path::Path::new(&path).exists() {
        panic!("file {path} does not exist");
    }
    let f = std::fs::File::open(&path).expect("cannot read file");
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    for _ in 0..n {
        let _ = reader.read_line(&mut line).expect("");
        print!("{line}");
        line = String::new();
    }
}
