fn print_item<T>(item: T) {
    println!("Here is your item: {item:?}");
}

fn main() {
    print_item(5);
}