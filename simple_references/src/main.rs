fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);

    reference_scope();
}

fn calculate_length(a_string: &String) -> usize {
    a_string.len()
}

fn reference_scope() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);
    println!("{}", r2.len());
    let mr3 = &mut s;
    //let mr4 = &mut s; // compile error, 同一個scope兩個mutable reference 
                      // second mutable borrow occurs here
    println!("{}", mr3);
    mr3.push_str(", world");
    //println!("{}, {}", r1, r2); // mutable borrow候用會compile error
    println!("{}", mr3);
}
