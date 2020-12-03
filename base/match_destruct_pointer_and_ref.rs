fn main() {
    let reference = &4;

    println!("reference: {:?}", reference);
    match reference {
        &1 => println!("is one!"),
        &val => println!("Got a value via destructuring: {:?}", val),
    }
    match *reference {
        1 | 2 => println!("one or two"),
        val => println!("Got a value via dereferencing: {:?}", val),
    }
    let ref reference_2 = 3;
    match reference_2 {
        &1 => println!("is one!"),
        &val => println!("Got a value via destructuring: {:?}", val),
    }
    match *reference_2 {
        1 | 2 => println!("one or two"),
        val => println!("Got a value via dereferencing: {:?}", val),
    }
    let mut mut_value = 6;
    println!("mut_value: {:?}", mut_value);
    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10, mut_value: {:?}", m);
        },
    }
    println!("mut_value: {:?}", mut_value);
}