fn main() {
    let x = 5u32;

    let y = {
        let x_square = x * x;
        let x_cube = x_square * x;

        x_cube + x_square + x
    };

    let z = {
        2 * x;
    };

    println!("x: {:?}", x);
    println!("y: {:?}", y);
    println!("z: {:?}", z);
}