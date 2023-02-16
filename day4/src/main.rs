use std::any;
use std::fmt;

fn print_type<T: fmt::Display>(item: T) {
    println!("{} is {}",item, any::type_name::<T>());
}

fn main() {
    print_type("Hello, world!")
}
