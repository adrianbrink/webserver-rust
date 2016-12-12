pub fn add_two(a: i32) -> i32 {
    a + 2
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test_works() {
        assert_eq!(4, add_two(2));
    }
}