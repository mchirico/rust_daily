pub mod submodule2;

use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

// Define a global vector to store the integers
lazy_static! {
    static ref INTEGER_VECTOR: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(Vec::new()));
}

// Add an integer to the global vector
#[allow(dead_code)]
pub fn add_integer(integer: i32) {
    let mut integer_vector = INTEGER_VECTOR.lock().unwrap();
    integer_vector.push(integer);
}

// Get a reference to the global vector
#[allow(dead_code)]
pub fn get_integer_vector() -> Arc<Mutex<Vec<i32>>> {
    Arc::clone(&INTEGER_VECTOR)
}
