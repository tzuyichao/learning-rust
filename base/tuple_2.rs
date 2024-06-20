fn main() {
    let tuple_of_three = ("one", "two", "three");
    let (a, b, c) = tuple_of_three;

    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);

    let (_, d, _) = tuple_of_three;
    println!("{:?}", d);
}
