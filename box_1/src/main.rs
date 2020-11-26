#[derive(Debug)]
enum List {
    // Cons(i32, List),
    Cons(i32, Box<List>),
    Nil,
}

// 2 | enum List {
//   | ^^^^^^^^^ recursive type has infinite size

use crate::List::{Cons, Nil};

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list: {:?}", list);
}
