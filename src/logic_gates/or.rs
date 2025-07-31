use crate::logic_gates::nand::Nand;
use crate::logic_gates::not::Not;

/**
 * Or gate:
 * if (a or b) out = 1, else out = 0
 */

pub struct Or;

impl Or {
    pub fn or(a: bool, b: bool) -> bool {
        let not_a = Not::not(a);
        let not_b = Not::not(b);
        Nand::nand(not_a, not_b)
    }
}

#[cfg(test)]
mod test_or {
    use super::*;

    #[test]
    fn test_or_a_is_true_and_b_is_false() {
        assert_eq!(Or::or(true, false), true);
    }

    #[test]
    fn test_or_a_is_true_and_b_is_true() {
        assert_eq!(Or::or(true, true), true);
    }

    #[test]
    fn test_or_a_is_false_and_b_is_true() {
        assert_eq!(Or::or(false, true), true);
    }

    #[test]
    fn test_or_a_is_false_and_b_is_false() {
        assert_eq!(Or::or(false, false), false);
    }
}