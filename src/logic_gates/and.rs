use crate::logic_gates::nand::Nand;
use crate::logic_gates::not::Not;

/**
 * And gate:
 * if (a and b) out = 1, else out = 0
 */
pub struct And;

impl And {
    pub fn and(a: bool, b: bool) -> bool {
        let nand_out = Nand::nand(a, b);
        Not::not(nand_out)
    }
}

#[cfg(test)]
mod test_and {
    use super::*;
    #[test]
    fn test_and_a_is_true_and_b_is_true() {
        assert_eq!(And::and(true, true), true);
    }

    #[test]
    fn test_and_a_is_true_and_b_is_false() {
        assert_eq!(And::and(true, false), false);
    }

    #[test]
    fn test_and_a_is_false_and_b_is_true() {
        assert_eq!(And::and(false, true), false);
    }

    #[test]
    fn test_and_a_is_false_and_b_is_false() {
        assert_eq!(And::and(false, false), false);
    }
}