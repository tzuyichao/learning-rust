macro_rules! calculate {
    (eval $e:expr) => {{
        {
            let val: usize = $e;
            println!("{} = {}", stringify!($e), val);
        }
    }}
}

fn main() {
    println!("Hello, world!");
    calculate! {
        eval 1 + 2
    }
    calculate! {
        eval (1 + 2) * (3 / 4)
    }
}
