fn main() {
    // vec![init value; length]
    let mut v2: Vec<u32> = vec![1; 10];
    println!("v2: {:?}", v2);
    v2.push(5);
    println!("v2: {:?}", v2);
}