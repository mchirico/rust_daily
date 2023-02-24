mod my_module;
use my_module::submodule1::add;
use my_module::submodule1::add2;
use my_module::my_module2::submodule2::add3;


#[allow(dead_code)]
fn main() {
    let sum = add(1, 2);
    let sum2 = add2(1, 2);
    let sum3 = add3(1, 2);

    println!("Sum: {}", sum);
    println!("Sum2: {}", sum2);
    println!("Sum3: {}", sum3);

}
