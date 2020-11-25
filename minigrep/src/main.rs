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
    if args.len() < 3 {
        panic!("not enough arguments");
    }
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}
