use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    println!("scores: {:?}", scores);

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("scores: {:?}", scores);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).unwrap();
    println!("{}: {}", team_name, score);

    let green_team_name = String::from("Green");
    let green_team_score_option = scores.get(&green_team_name);
    match green_team_score_option {
        None => println!("Green Team Not found"),
        Some(green_team_score) => println!("Green team score: {}", green_team_score),
    }

    println!("=== Iterate HashMap:");
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.entry(String::from("Green")).or_insert(30);
    scores.entry(team_name).or_insert(30);
    println!("{:?}", scores);
}
