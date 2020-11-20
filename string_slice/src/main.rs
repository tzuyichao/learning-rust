fn main() {
    let mut text = String::from("hello a world");
    let first_word_index = first_word(&text);
    println!("first word end index: {}", first_word_index);
    let first_word = &text[..first_word_index];
    println!("first word: {}", first_word);
    text.clear();
    // println!("after clear text, first word: {}", first_word); // compile error
        // text.clear();
        // ^^^^^^^^^^^^ mutable borrow occurs here
        // println!("after clear text, first word: {}", first_word);
        //                                              ---------- immutable borrow later used here
    
    let text2 = String::from("hello a world");
    let first_word2 = first_word_enhancement(&text2);
    println!("{}", first_word2);
    // let first_word2 = first_word_enhancement(&text2[..]);
    //                                          ^^^^^^^^^^ expected struct `std::string::String`, found `str`
    let _first_word = first_word_enhancement2(&text2);            // immutable reference
    let _first_word = first_word_enhancement2(&text2[..]);        // string slice
    let _first_word = first_word_enhancement2("hello a world");   // string literal
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_enhancement(s: &String) -> &str {
    let bytes = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn first_word_enhancement2(s: &str) -> &str {
    let bytes = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}