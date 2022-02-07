mod adder;

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adder() {
        assert_eq!(3, adder::adder(1, 2));
    }

}
