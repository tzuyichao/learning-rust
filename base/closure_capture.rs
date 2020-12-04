use std::mem;

fn main() {
    let color = String::from("green");

    let print = || -> () {
        println!("`color`: {}", color);
    };

    print();

    let moveable = Box::new(3);

    let consume = || {
        println!("`moveable`: {:?}", moveable);
        mem::drop(moveable);
    };

    consume();
    //consume();
}