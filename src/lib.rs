pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_add_with_negative() {
        assert_eq!(add(-1, -2), -3);
    }

    #[test]
    fn test_sub() {
        assert_eq!(sub(6, 3), 3);
    }

    #[test]
    fn test_sub_with_negative() {
        assert_eq!(sub(-15, -7), -8)
    }
}
