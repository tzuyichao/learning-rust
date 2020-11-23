#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("The area of the rectangle is {} square pixels.", area(width1, height1));
    println!("Hello, world!");

    let rect1 = (30, 50);
    println!("The area of the rectangle is {} square pixels.", area_tuple(rect1));

    let rectangle1 = Rectangle {
        height: 50,
        width: 30,
    };
    println!("Rectangle1: {:?}", rectangle1);
    println!("The area of the rectangle is {} square pixels.", area_rectangle(rectangle1));
    // 如果在這裡println!會compile error，因為上一個method已經拿走ownership

    let rectangle2 = Rectangle {
        height: 50,
        width: 30,
    };
    
    println!("The area of the rectangle is {} square pixels.", area_rectangle2(&rectangle2));
    println!("Rectangle2: {:?}", rectangle2);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_rectangle(rectangle: Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn area_rectangle2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}