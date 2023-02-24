use std::env;

struct Person {
    name: String,
    age: u32,
}

trait Foo {
    fn describe(&self) -> String;
}

impl Foo for Person {
    fn describe(&self) -> String {
        format!("{} is {} years old", self.name, self.age)
    }
}

#[allow(dead_code)]
fn read_args() {
    for (key, value) in env::args().enumerate() {
        println!("{}: {}", key, value);
    }
}

fn main() {
    println!("Hello, world!");
    let p = Person {
        name: "John".to_string(),
        age: 32,
    };
    println!("{}", p.describe());
}
