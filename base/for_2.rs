fn main() {
    let names: Vec<&str> = vec!["Bob", "Frank", "Ferris"];

    println!("iter() =====");
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us"),
            _ => println!("Hello {}", name),
        }
    }
    println!("into_iter() =====");
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us"),
            _ => println!("Hello {}", name),
        }
    }
    let mut names: Vec<&str> = vec!["Bob", "Frank", "Ferris"];
    println!("into_mut() =====");
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us",
            _ => "Hello",
        }
    }
    println!("names: {:?}", names);
}