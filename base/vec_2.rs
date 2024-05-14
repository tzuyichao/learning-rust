fn main() {
    let name1 = String::from("Windy");
    let name2 = String::from("Gomesy");
    let mut my_vec = Vec::new();
    my_vec.push(name1);
    my_vec.push(name2);
    println!("{:?}", my_vec);
    let my_vec2 = vec!["test", "one", "two"];
    println!("{:?}", my_vec2);
}
