fn main() {
    let array_of_ten = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let two_to_five = &array_of_ten[2..5];
    let start_at_one = &array_of_ten[1..];
    let end_of_five = &array_of_ten[..5];
    let everything = &array_of_ten[..];

    println!("Two to Five: {two_to_five:?}, 
Start at one: {start_at_one:?},
End of Five: {end_of_five:?},
Everything: {everything:?}");
}