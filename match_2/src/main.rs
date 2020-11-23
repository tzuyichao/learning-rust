fn main() {
    let some_u8_value = Some(0u8);
    if let Some(1) = some_u8_value {
        println!("one");
    } else if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("No");
    }
    println!("Done");
}
