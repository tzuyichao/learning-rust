fn return_item<MyType>(item: MyType) -> MyType {
    println!("Here is your item.");
    item
}

fn main() {
    let item = return_item(5);
    println!("item: {item}");
}