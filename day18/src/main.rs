macro_rules! create_function {
    // This macro takes an argument of designator `ident` and
    // creates a function named `$func_name`.
    // The `ident` designator is used for variable/function names.
    ($func_name:ident) => {
        fn $func_name(u: u32) {
            // The `stringify!` macro converts an `ident` into a string.
            println!("You called \"{}({})\"", stringify!($func_name), u);
        }
    };
}

create_function!(foo);

fn main() {
    foo(3);
    println!("Hello, world!");
}
