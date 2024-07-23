use std::collections::HashMap;

fn roman_to_int(s: String) -> i32 {
    let roman_map: HashMap<char, i32> = [
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ].iter().cloned().collect();

    let mut result = 0;
    let mut prev_value = 0;

    for c in s.chars().rev() {
        let value = roman_map.get(&c).unwrap_or(&0);

        if *value < prev_value {
            result -= value;
        } else {
            result += value;
        }
        prev_value = *value;
    }
    result
}

fn main() {
    let s1: String = String::from("LVIII");
    println!("{}", roman_to_int(s1));
}