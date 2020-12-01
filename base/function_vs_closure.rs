fn main() {
    let out = 40;
    // fn add(i: i32, j: i32) -> i32 { i + j + out }
    // 3 |     fn add(i: i32, j: i32) -> i32 { i + j + out }
    //   |                                             ^^^
    //   |
    //   = help: use the `|| { ... }` closure form instead
    fn add(i: i32, j: i32) -> i32 { i + j }
    let closure1 = |i: i32, j: i32| -> i32 { i + j + out };
    let i = 10;
    let j = 5;
    println!("add: {}", add(i, j));
    println!("closure1: {}", closure1(i, j));
}