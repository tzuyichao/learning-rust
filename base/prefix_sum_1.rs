fn print(n: i32) {
    let mut sum = 0;
    print!("1 to {} prefix sum: ", n);
    for i in 1..=n {
        sum += i;
        print!("{} ", sum);
    }
    println!();

    sum = 0;
    print!("{} to 1 prefix sum: ", n);
    for i in (1..=n).rev() {
        sum += i;
        print!("{} ", sum);
    }
    println!();
}

fn pivot_integer(n: i32) -> i32 {
    if n == 1 {
        return 1;
    }
    let mut l2r = Vec::new();
    let mut r2l = Vec::new();
    let mut sum = 0;
    for i in 1..=n {
        sum += i;
        l2r.push(sum);
    }
    sum = 0;
    for i in (1..=n).rev() {
        sum += i;
        r2l.push(sum);
    }
    for i in 1..n {
        if r2l[(n-i) as usize] == l2r[(i-1) as usize] {
            return i;
        }
    }
    -1
}

fn main() {
    print(8);
    println!("{}", pivot_integer(8));
}