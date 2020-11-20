fn main() {
    let x = 5;
    let _y = x;
    println!("{}", x);
    let s1 = String::from("hello");
    let _s2 = s1;    // compile error
    println!("{}", s1);
}
