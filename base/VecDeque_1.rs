use std::collections::VecDeque;

fn main() {
    let mut buf:VecDeque<u32> = VecDeque::new();
    buf.push_front(1);
    buf.push_front(2);
    println!("buf: {:?}", buf);
    buf.push_back(3);
    buf.push_back(4);
    buf.push_back(5);
    println!("buf: {:?}", buf);
    assert_eq!(buf.get(0), Some(&2));
}