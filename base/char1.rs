fn main() {
    let my_number: u8 = 100;
    println!("{}", my_number as char);
    
    println!("Size of a char: {}", std::mem::size_of::<char>());
    println!("Size of a: {}", "a".len());
    println!("Size of ß: {}", "ß".len());
    println!("Size of 国: {}", "国".len());
    println!("Size of 𓅱: {}", "𓅱".len());
    println!("Size of 'Hello!': {}", "Hello!".len());
    println!("Size of '안녕!': {}", "안녕!".len());
}
