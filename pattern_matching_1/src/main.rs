fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    let mut stack: Vec<u8> = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c', 'd'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    //  let (x, y, z) = (1, 2, 3, 4);
//|         ^^^^^^^^^   ------------ this expression has type `({integer}, {integer}, {integer}, {integer})`
//|         |
//|         expected a tuple with 4 elements, found one with 3 elements
    let (x, y, z) = (1, 2, 3);
    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);

    let point = (5, 10);
    print_coordinates(&point);
    println!("{:?}", point);

    // match starts a new scope
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);

    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    // destructuring struct
    let p = Point {
        x: 0,
        y: 7,
    };
    let Point {x: a, y: b} = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
    let Point {x, y} = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
    match p {
        Point {x, y:0} => println!("On the x axis at {}", x),
        Point {x: 0, y} => println!("On the y axis at {}", y),
        Point {x, y} => println!("On neither axis: ({}, {})", x, y),
    }
    let origin  = Point3D {x: 0, y: 0, z: 0};
    match origin {
        Point3D {x: 1, y, z} => println!("y is {}, z is {}", y, z),
        Point3D {x, ..} => println!("x is {}", x),
    }

    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}

enum Message {
    Hello {id: i32},
}

struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

struct Point {
    x: i32,
    y: i32,
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current Location: ({}, {})", x, y);
}

