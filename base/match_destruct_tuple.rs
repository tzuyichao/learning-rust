fn main() {
    let triple = (0, "abc", true);

    println!("Tell me about: {:?}", triple);
    match triple {
        (0, y, z) => println!("first is `0`, y is {:?} and z is {:?}", y, z),
        (1, y, z) => println!("first is `1`, y is {:?} and z is {:?}", y, z),
        _ => println!("It doesn't matter what they are"),
    }
}