fn main() {
    let v: Vec<u32> = Vec::new();
    // v.push(1);
    // v.push(1);
    // |     ^ cannot borrow as mutable
    println!("v: {:?}", v);

    let mut v1: Vec<u32> = Vec::new();
    v1.push(1);
    v1.push(3);
    println!("v1: {:?}", v1);

    // use macro
    let v2: Vec<u32> = vec![1, 3, 5, 7, 9];
    println!("v2: {:?}", v2);
    let third: &u32 = &v2[2];
    println!("The third element is {}", third);
    match v2.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    let mut v3: Vec<u32> = vec![1, 3, 4, 5, 6];
    let first = &v3[0];

//    v3.push(10);
// 26 |     v3.push(10);
//    |     ^^^^^^^^^^^ mutable borrow occurs here
// 27 |
// 28 |     println!("The first element is {}", first);
//    |                                         ----- immutable borrow later used here

    println!("The first element is {}", first);
    v3.push(10);
    println!("v3: {:?}", v3);
    let _first = &v3[0];
    v3.push(15);
    println!("v3: {:?}", v3);

    let v4: Vec<i32> = vec![100, 32, 57];
    for i in &v4 {
        println!("{}", i);
    }
}
