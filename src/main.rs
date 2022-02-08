mod adder;

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adder() {
        assert_eq!(3, adder::adder(1, 2));
        assert_eq!(7, adder::adder(5, 2));
        assert_eq!(8, adder::adder(6, 2));
    }

}
