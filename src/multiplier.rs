// fn multiplier(a: u32, b: u32) -> u32;
use crate::adder::{self, adder};

pub fn mult_rec(a: u32, b:u32, mult: u32) -> u32{
    if b == 0 {
        return a;
    }
    mult_rec(adder(a, mult), b - 1, mult)
}

pub fn multiplier(a: u32, b: u32) -> u32 {
    mult_rec(a, b - 1, a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(6, multiplier(3, 2));
        assert_eq!(8, multiplier(4, 2));
        assert_eq!(10, multiplier(5, 2));
    }
}
