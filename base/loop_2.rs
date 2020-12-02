#![allow(unreachable_code)]
fn main() {
    'outer: loop {
        println!("Entered the outer loop");
        'inner: loop {
            println!("Enter the inner loop");
            break 'outer;
        }
        println!("This point will never reached");
    }
    println!("Exited the outer loop");
}