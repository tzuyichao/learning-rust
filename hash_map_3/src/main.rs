use std::collections::HashMap;

fn main() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        // mutable reference
        *count += 1;
    }
    println!("Hello, world! {:?}", map);
}
