fn main() {
    let s = String::from("hello");
    println!("before call take_ownership: {}", s); // ok
    take_ownership(s);
    //println!("{}", s);    // compile error
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}