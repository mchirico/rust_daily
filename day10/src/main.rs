fn update(input: &mut i32) {
    *input += 3;
}

fn main() {
    println!("Hello, world!");
}
#[cfg(test)]
mod tests {
    use crate::update;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works2() {
        let result = 2 + 3;
        assert_eq!(result, 5);
    }

    #[test]
    fn test_update() {
        let mut result = 2;
        update(&mut result);
        assert_eq!(result, 5);
    }
}
