macro_rules! five_times {
    ($x:expr) => (5 * $x);
}

fn main() {
    assert_eq!(55, five_times!(2 + 3*3));
}
