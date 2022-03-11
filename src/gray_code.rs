pub fn gray_code(n: u32) -> u32 {
    return return n ^ (n >> 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(0, gray_code(0));
        assert_eq!(3, gray_code(2));
        assert_eq!(2, gray_code(3));
        assert_eq!(6, gray_code(4));
        assert_eq!(7, gray_code(5));
    }
}
