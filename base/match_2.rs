fn main() {
    let my_number: u8 = 5;

    match my_number {
        0 => println!("it's zero"),
	1 => println!("it's one"),
	2 => println!("it's two"),
	_ => println!("it's some other number"),
    }

    let second_number = match my_number {
        0 => 0,
	5 => 10,
	_ => 2,
    };
    println!("second numner: {second_number}");
}
