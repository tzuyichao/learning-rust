fn destroy_box(c: Box<u32>) {
    println!("Destroying a box that contains {}", c);
    // c is destroyed and the memory freed
}

fn main() {
    let x = 5u32;

    let y = x;

    println!("x is {}, and y is {}", x, y);

    let a = Box::new(5u32);
    println!("a contains: {}", a);
    let b = a;
    // compile error
    // println!("a contains: {}", a);
    destroy_box(b);
    // compile error
    //println!("b contains: {}", b);
}