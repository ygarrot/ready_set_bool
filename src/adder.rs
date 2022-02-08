use rand::Rng;

// fn adder(a: u32, b: u32) -> u32;

// pub fn inc(i: u32) -> u32 {
//     if (i & 1) == 0 {
//         return i | 1;
//     }
//     return inc(i >> 1) << 1;
// }

// pub fn adder(a: u32, b: u32) -> u32 {
//     if b == 0 {
//         return a;
//     }
//     adder(inc(a), b - 1)
// }

pub fn adder(a: u32, b: u32) -> u32 {
    let mut x = a;
    let mut carry;
    for _ in 0..b {
        carry = 1;
        while (x & carry) != 0 {
            x &= !carry;
            carry <<= 1;
        }
        x |= carry;
    }
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut rng = rand::thread_rng();


        for _ in 0..16 {
            let a: u32 = rng.gen_range(0..10);
            let b: u32 = rng.gen_range(0..10);
            println!("{} + {}, ", a, b);
            assert_eq!(a + b, adder(a, b));
        }

        assert_eq!(15, adder(8, 7));
        // assert_eq!(7, adder(5, 2));
        // assert_eq!(8, adder(6, 2));

        // assert_eq!(7, adder(5, 3));
        // assert_eq!(8, adder(7, 16));
    }

}
