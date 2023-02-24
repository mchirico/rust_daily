mod math;
#[allow(dead_code)]
fn update(input: &mut i32) {
    *input += 3;
}

fn main() {
    println!("Hello, world!");
}
#[cfg(test)]
mod tests {
    use crate::math;
    use crate::update;

    #[test]
    fn test_update() {
        let mut result = 2;
        update(&mut result);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_add3() {
        let mut result = 2;
        math::add3(&mut result);
        assert_eq!(result, 5);
    }
}
