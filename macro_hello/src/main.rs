macro_rules! say_hello {
    () => {
        println!("Hello!");
    }
}

macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("You called: {:?}()", stringify!($func_name));
        }
    }
}

macro_rules! create_function2 {
    ($func_name:ident, $base:literal) => {
        fn $func_name(size: u8) {
            for idx in 1..=$base*size {
                println!("You called: {:?}() {}", stringify!($func_name), idx);
            }
        }
    }
}

fn main() {
    say_hello!();
    create_function!(foo);
    create_function!(bar);
    create_function2!(goor, 3);

    foo();
    bar();
    goor(5);
}
