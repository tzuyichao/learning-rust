fn main() {
    let data = "initial data";
    let s: String = data.to_string();

    println!("data: {:?}", data);
    println!("s: {}", s);

    let mut s1: String = String::from("foo");
    let ss = "bar";
    s1.push_str(ss);
    println!("s1: {}", s1);

    let str1 = String::from("Hello, ");
    let str2 = String::from("world!");
    let str3 = str1 + &str2;
    println!("str3: {}", str3);
    // println!("str1: {}", str1);
//     17 |     println!("str1: {}", str1);
//        |                          ^^^^ value borrowed here after move
    println!("str2: {}", str2);

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");
    let str4 = format!("{}-{}-{}", tic, tac, toe);
    println!("str4: {}", str4);
}
