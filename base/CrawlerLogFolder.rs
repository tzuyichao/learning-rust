struct Solution {}

impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        let mut stack: Vec<String> = Vec::new();

        for log in logs.iter() {
            match log.as_str() {
                "./" => {},
                "../" => {
                    stack.pop();
                },
                _ => {
                    stack.push(log.to_string());
                }
            }
        }

        stack.len() as i32
    }
}

fn main() {
    println!("{}", Solution::min_operations(vec!["d1/".to_string(),"d2/".to_string(),"../".to_string(),"d21/".to_string(),"./".to_string()]));
}