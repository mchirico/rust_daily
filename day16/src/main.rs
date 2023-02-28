mod my_module;
#[allow(unused_imports)]
use crate::my_module::my_module2::{add_integer, get_integer_vector};
#[allow(unused_imports)]
use my_module::{my_module2::submodule2, submodule1};

#[allow(dead_code)]
fn main() {
    let mut s = String::from("hello");
    submodule1::change(&mut s);
    println!("s: {}", s);
    let result = submodule1::change_push_pop(&mut s);
    println!("s: {}, result: {:?}", s, result);
}

#[cfg(test)]
mod tests {

    use crate::my_module::my_module2::{add_integer, get_integer_vector};
    use crate::my_module::submodule1;

    #[test]
    fn test_push_pop() {
        let mut s = String::from("hello");
        let result = submodule1::change_push_pop(&mut s);
        println!("s: {}, result: {:?}", s, result);
        assert_eq!(s, "hell, world");
        assert_eq!(result, Some('o'));
    }

    #[test]
    fn test_string_param_change() {
        let mut s = String::from("hello");
        submodule1::change_param(&mut s, ", spud");
        assert_eq!(s, "hello, spud");
        submodule1::change_param(&mut s, ", spud");
        assert_eq!(s, "hello, spud, spud")
    }

    #[test]
    fn test_string_change() {
        let mut s = String::from("hello");
        submodule1::change(&mut s);
        assert_eq!(s, "hello, world");
    }

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
