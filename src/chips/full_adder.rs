use crate::chips::half_adder::HalfAdder;
use crate::logic_gates::or::Or;

/**
 * Computes the sum of three bits.
 */
pub struct FullAdder;

impl FullAdder {
    pub fn full_adder(a: bool, b: bool, c: bool) -> (bool, bool) {
        let (a_plus_b, a_b_carry) = HalfAdder::half_adder(a, b);
        let (sum, carry_c) = HalfAdder::half_adder(a_plus_b, c);
        let carry = Or::or(a_b_carry, carry_c);

        (sum, carry)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_adder_all_false() {
        assert_eq!(FullAdder::full_adder(false, false, false), (false, false))
    }

    #[test]
    fn test_full_adder_all_true() {
        assert_eq!(FullAdder::full_adder(true, true, true), (true, true))
    }

    #[test]
    fn test_full_adder_a_is_true_and_b_and_c_false() {
        assert_eq!(FullAdder::full_adder(true, false, false), (true, false))
    }

    #[test]
    fn test_full_adder_a_and_b_true_c_false() {
        assert_eq!(FullAdder::full_adder(true, true, false), (false, true))
    }

    #[test]
    fn test_full_adder_a_and_b_false_c_true() {
        assert_eq!(FullAdder::full_adder(false, false, true), (true, false))
    }

    #[test]
    fn test_full_adder_a_false_b_and_c_true() {
        assert_eq!(FullAdder::full_adder(false, true, true), (false, true))
    }
}