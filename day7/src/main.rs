use std::collections::HashMap;
fn main() {
    let mut day = HashMap::new();
    day.insert("Monday", 1);
    day.insert("Tuesday", 2);
    day.entry("Wednesday").or_insert(3);
    println!("Current days: {:?}", day);
    println!("Monday: {:?}", day.get("Monday"));
}
