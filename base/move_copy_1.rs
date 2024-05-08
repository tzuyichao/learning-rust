fn prints_number(number: i32) {
    println!("{}", number);
}

fn print_country(country: String) {
    println!("{}", country)
}
 
fn main() {
    let my_number = 8;
    prints_number(my_number);
    prints_number(my_number);
    
    let my_country = String::from("Taiwan"); // move occurs because `my_country` has type `String`, which does not implement the `Copy` trait
    print_country(my_country);
    //print_country(my_country); // value used here after move
}
