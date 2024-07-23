fn capitalize_title(title: String) -> String {
    let mut words: Vec<String> = title.split_whitespace().map(|s| s.to_string()).collect();

    for word in &mut words {
        let len = word.len();
        if len <= 2 {
            *word = word.to_lowercase();
        } else {
            *word = capitalize(word.to_string());
        }
    }
    words.join(" ")
}

fn capitalize(s: String) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(first) => {
            let first_upper = first.to_uppercase().collect::<String>();
            let rest_lower = c.as_str().to_lowercase();
            first_upper + &rest_lower
        },
    }
}

fn main() {
    println!("{}", capitalize_title(String::from("heLLo world")));

    println!("{}", capitalize_title(String::from("i lOve leetcode")));
}