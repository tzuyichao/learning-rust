use std::fmt;

fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");

    println!("The value of guess is: {}", guess);

    struct TTuple(i32, f64, u8);

    let ttup = TTuple(500, 6.4, 1);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    impl fmt::Display for TTuple {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {}, {})", self.0, self.1, self.2)
        }
    }
    println!("The value of ttup is: {}", format!("{}", ttup));
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
}
