struct Solution {
}

impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();
        for op in operations.iter() {
            let n = stack.len();
            if op == "+" {
                stack.push(stack[n-2] + stack[n-1]);
            } else if op == "D" {
                stack.push(2 * stack[n-1])
            } else if op == "C" {
                stack.pop();
            } else {
                stack.push(op.parse().expect("Not a valid number"));
            }

        }
        stack.iter().sum()
    }
}

fn main() {
    println!("{}", Solution::cal_points(vec!["5".to_string(),"2".to_string(),"C".to_string(),"D".to_string(),"+".to_string()]));
}