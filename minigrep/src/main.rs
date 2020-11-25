use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);
    
    println!("Query: {}", config.query);
    println!("In file: {}", config.filename);
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = &args[1];
    let filename = &args[2];

    Config { query: query.to_string(), filename: filename.to_string() }
}
