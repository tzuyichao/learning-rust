fn main() {
    struct TextMatch(usize, String);

    let m: TextMatch = TextMatch(12, "needle".to_owned());

    assert_eq!(m.0, 12);
    assert_eq!(m.1, "needle");
}
