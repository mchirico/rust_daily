#[allow(dead_code)]
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}
#[allow(dead_code)]
pub fn add2(x: i32, y: i32) -> i32 {
    x + y
}

#[allow(dead_code)]
pub fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

#[allow(dead_code)]
pub fn change_param(some_string: &mut String, s: &str) {
    some_string.push_str(s);
}
