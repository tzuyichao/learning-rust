use std::collections::HashMap;

fn main() {
    // let map1 = HashMap::new();
    // map1.insert(String::from("key"), 10);
// 5 |     map1.insert(String::from("key"), 10);
//   |     ^^^^ cannot borrow as mutable

    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);

    println!("Hello, world! {:?}", scores);

    let teams = vec![String::from("blue"), String::from("yellow")];
    let initial_scores = vec![10, 50];

    let mut scores2: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("scores2: {:?}", scores2);
    scores2.insert(String::from("green"), 15);

    // ownership
    let field_name = String::from("Favorite Color");
    let field_value = String::from("Blue");
    let mut favorites = HashMap::new();
    favorites.insert(field_name, field_value);
    println!("{:?}", favorites);
    // let str1 = format!("{}: {}", field_name, field_value);
    // println!("str1: {}", str1);
    // field_name and field_value are "value borrowed here after move"
}
