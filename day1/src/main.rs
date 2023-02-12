use std::env;
fn read_args() {
    for (key, value) in env::args().enumerate() {
        println!("{}: {}", key, value);
    }
}

fn main() {
    read_args();

}

