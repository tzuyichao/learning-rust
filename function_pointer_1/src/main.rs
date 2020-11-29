fn main() {
    println!("Hello, world!");
    let answer = do_twice(add_one, 10);
    println!("Answer: {}", answer);

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    println!("list_of_strings: {:?}", list_of_strings);

    let list_of_strings2: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
    println!("list_of_strings2: {:?}", list_of_strings2);

    fn returns_closure() -> fn(i32) -> i32 {
        |x| x + 1
    }

    let answer = do_twice(returns_closure(), 5);
    println!("answer: {}", answer);
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}