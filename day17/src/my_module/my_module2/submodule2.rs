#[allow(dead_code)]
pub fn add(x: i32, y: i32) -> i32 {
    println!("in add submodule2");
    x + y
}

#[allow(dead_code)]
pub struct Binding2 {
    pub(crate) integer_vector: Vec<i32>,
}

#[allow(dead_code)]
impl Binding2 {
    pub(crate) fn new() -> Binding2 {
        Binding2 {
            integer_vector: Vec::new(),
        }
    }
}
