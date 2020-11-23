fn main() {
    let hello = String::from("Hola");               // 4
    println!("len of hello is {}", hello.len());
    let hello = String::from("Здравствуйте");       // 24
    println!("len of hello is {}", hello.len());

    let s = &hello[0..4];
    println!("s: {}", s);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
