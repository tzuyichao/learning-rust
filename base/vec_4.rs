fn main() {
    let my_vec: Vec<u8> = [1, 2, 3].into();
    let my_vec2: Vec<_> = [9, 0, 1].into();
    println!("{:?}", my_vec);
    println!("{:?}", my_vec2);
    println!("{:?}", my_vec2);
}
