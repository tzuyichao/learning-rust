fn main() {
    let a: i32 = 8;
    let b: i32 = 9;
    let str = String::from("532");

    println!("8 & 1: {}", a & 1);
    println!("9 & 1: {}", b & 1);
    for c in str.chars().rev() {
        println!("{}", c);
    }
}