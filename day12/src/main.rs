mod my_module;
use my_module::submodule1::add;

fn main() {
    let sum = add(1, 2);
    println!("Sum: {}", sum);
}
