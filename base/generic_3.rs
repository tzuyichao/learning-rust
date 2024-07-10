use std::fmt::Debug;

fn print_item<T: Debug>(item: T) {
    println!("Here is your item: {item:?}");
}

fn main() {
    print_item(5);
}