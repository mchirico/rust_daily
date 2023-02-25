mod my_module;
use my_module::{my_module2::submodule2, submodule1};

#[allow(dead_code)]
fn main() {
    let sum = submodule1::add(1, 2);
    let sum2 = submodule1::add2(1, 2);
    let sum3 = submodule2::add(1, 2);

    println!("Sum: {}", sum);
    println!("Sum2: {}", sum2);
    println!("Sum3: {}", sum3);
}
