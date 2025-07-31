use crate::logic_gates::and::And;
use crate::logic_gates::not::Not;
use crate::logic_gates::or::Or;

/**
 * Exclusive-or gate:
 * if ((a and Not(b)) or (Not(a) and b)) out = 1, else out = 0
 */
pub struct Xor;

impl Xor {
    pub fn xor(a: bool, b: bool) -> bool {
        let not_a = Not::not(a);
        let not_b = Not::not(b);
        let and_a_not_b = And::and(a, not_b);
        let and_b_not_a = And::and(not_a, b);
        Or::or(and_a_not_b, and_b_not_a)
    }
}

#[cfg(test)]
mod tests_xor {
    use super::*;

    #[test]
    fn test_xor_a_is_false_and_b_is_false() {
        assert_eq!(Xor::xor(false, false), false);
    }

    #[test]
    fn test_xor_a_is_true_and_b_is_false() {
        assert_eq!(Xor::xor(true, false), true);
    }

    #[test]
    fn test_xor_a_is_true_and_b_is_true() {
        assert_eq!(Xor::xor(true, true), false);
    }

    #[test]
    fn test_xor_a_is_false_and_b_is_true() {
        assert_eq!(Xor::xor(false, true), true);
    }
}