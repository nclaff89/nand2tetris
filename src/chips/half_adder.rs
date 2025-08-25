use crate::logic_gates::and::And;
use crate::logic_gates::xor::Xor;

pub struct HalfAdder;

impl HalfAdder {
    pub fn half_adder(a: bool, b: bool) -> (bool, bool) {
        let sum = Xor::xor(a, b);
        let carry = And::and(a, b);
        (sum, carry)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_half_adder_a_and_b_are_false() {
        assert_eq!(HalfAdder::half_adder(false, false), (false, false));
    }

    #[test]
    fn test_half_adder_a_and_b_are_true() {
        assert_eq!(HalfAdder::half_adder(true, true), (false, true))
    }

    #[test]
    fn test_half_adder_a_is_true_and_b_is_false() {
        assert_eq!(HalfAdder::half_adder(true, false), (true, false));
    }

    #[test]
    fn test_half_adder_a_is_false_and_b_is_true() {
        assert_eq!(HalfAdder::half_adder(false, true,), (true, false));
    }
}