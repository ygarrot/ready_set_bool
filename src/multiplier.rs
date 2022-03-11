// fn multiplier(a: u32, b: u32) -> u32;
use rand::Rng;
use crate::adder::adder_v1;

pub fn multiplier(mut a: u32, mut b: u32) -> u32 {
    let mut tmp = 1;
    let mut square = 0;

    while tmp < b {
        tmp <<= 1;
        square +=1;
    }
    println!("{} * {} = {:?}", a, b, (a << square));
    return (a << square) - a * (tmp - b);
}

// pub fn multiplier(mut a: u32, mut b: u32) -> u32 {
//     let mut ret = 0;

//     if b == 0 {
//         return 0;
//     }
//     while b > 0 {
//         if (b & 1) == 1 {
//             ret = adder_v1(ret, a)
//         }
//         a <<= 1;
//         b >>= 1;
//     }
//     return ret;
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        println!("========================");
        println!("multiplier");
        assert_eq!(6, multiplier(3, 2));
        assert_eq!(8, multiplier(4, 2));
        assert_eq!(10, multiplier(5, 2));
        assert_eq!(9, multiplier(3, 3));
        assert_eq!(42, multiplier(6, 7));

       for _ in 0..16 {
           let mut rng = rand::thread_rng();
            let a: u32 = rng.gen_range(0..10);
            let b: u32 = rng.gen_range(0..10);
            assert_eq!(a * b, multiplier(a, b));
        }
    }
}
