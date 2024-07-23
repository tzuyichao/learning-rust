fn main() {
    let fruit = vec!['1', '2', '3'];

    let buffer_overflow = fruit[4];
    assert_eq!(buffer_overflow, '4');
}
