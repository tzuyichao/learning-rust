struct Solution {}

impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let n = mat.len();
        let mut ans: i32 = 0;
        for i in 0..n {
            for j in 0..n {
                if i == j || i+j == n-1 {
                    ans += mat[i][j]
                }
            }
        }
        ans
    }
}

fn main() {
    let mat = vec![vec![1, 1, 1, 1], vec![1, 1, 1, 1], vec![1, 1, 1, 1], vec![1, 1, 1, 1]];
    println!("{}", Solution::diagonal_sum(mat));
}