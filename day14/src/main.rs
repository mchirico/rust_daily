mod my_module;
use crate::my_module::my_module2::{add_integer, get_integer_vector};
use my_module::{my_module2::submodule2, submodule1};

include!(concat!(env!("OUT_DIR"), "/hello.rs"));

#[allow(dead_code)]
fn main() {
    let sum = submodule1::add(1, 2);
    let sum2 = submodule1::add2(1, 2);
    let sum3 = submodule2::add(1, 2);
    add_integer(10);
    add_integer(20);
    add_integer(30);

    println!("Sum: {}", sum);
    println!("Sum2: {}", sum2);
    println!("Sum3: {}", sum3);
    println!("result {}", get_integer_vector().lock().unwrap()[0]);

    println!("{}", message());
}

#[cfg(test)]
mod tests {

    use crate::my_module::my_module2::{add_integer, get_integer_vector};

    #[test]
    fn test_get_integer_vector() {
        add_integer(10);
        add_integer(20);
        add_integer(30);

        let binding = get_integer_vector();
        let integer_vector = binding.lock().unwrap();

        assert_eq!(integer_vector.len(), 3);
    }
}
