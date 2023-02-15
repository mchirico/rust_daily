use std::fmt;
fn compare_and_print<T, U>(a: T, b: U)
where
    T: fmt::Display + PartialEq + From<U>,
    U: fmt::Display + PartialEq + Copy,
{
    if a == T::from(b) {
        println!("{} and {} are equal", a, b);
    } else {
        println!("{} and {} are not equal", a, b);
    }
}
fn main() {
    compare_and_print(1.0, 1);
    compare_and_print(1.1, 1);
    compare_and_print("hello", "hello");
    compare_and_print("hello", "world");
}
