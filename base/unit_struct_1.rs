struct Empty;

fn main() {
    let x = Empty;
    println!("adress of x: {:p}", &x);
    let y = x;
    println!("adress of y: {:p}", &y);
    let z = Empty;
    println!("address of z:{:p}", &z);
    //assert_eq!((...), std::ops::RangeFull);
}