fn main() {
    let s = String::from("Hello Rust world!");

    let words: Vec<&str> = s.split_whitespace().collect();

    for word in words.iter() {
        let len = word.len();
        println!("{} len: {}", word, len);
    }
}