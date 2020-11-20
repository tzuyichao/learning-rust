fn main() {
    let s1 = give_ownership();

    let s2 = String::from("hello");

    let s3 = take_and_gives_back(s2);

    println!("s1: {}", s1);
    println!("s3: {}", s3);
    //println!("s2: {}", s2);    // compile error
}

fn give_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn take_and_gives_back(some_string: String) -> String {
    some_string
}