use sha2::{Sha256, Digest};
use hex;
use std::env;
use std::io::{self, Read};

fn compute_sha256(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = if args.len() > 1 {
        args[1].clone()
    } else {
        let mut input_str = String::new();
        io::stdin().read_to_string(&mut input_str).expect("Fail to read from stdin");
        input_str.trim().to_string()
    };
    
    let hash_value = compute_sha256(&input);
    println!("{}", hash_value);
}
