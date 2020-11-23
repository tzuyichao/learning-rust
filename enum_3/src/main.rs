#[derive(Debug)]
enum Message {
    Quit,
    Move {x:i32, y:i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Write(msg) => println!("{}", msg),
            Message::Quit => println!("Quit"),
            Message::Move {x, y} => println!("move to {}, {}", x, y),
            Message::ChangeColor(r, g, b) => println!("Change color to {}, {}, {}", r, g, b),
        }
    }
}

fn main() {
    println!("Hello, world!");
    let m = Message::Write(String::from("hello"));
    m.call();
    let m = Message::Quit;
    m.call();
    let m = Message::Move {x: 1, y: 2};
    m.call();
    let m = Message::ChangeColor(244, 244, 244);
    m.call();
}
