fn main() {
    let t = 5;
    let s = t;

    println!("t: {t}");
    
    let s1 = String::from("hello");
    let s2 = s1;
    // compile error: String沒有Copy trait
    // println!("s1: {s1}");
}
