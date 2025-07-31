/***
* Nand gate:
* if Not(a) and Not(b) out = 1, else out = 0
 */
pub struct Nand;

impl Nand {
    pub fn nand(a: bool, b: bool) -> bool {
        !(a && b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nand_a_and_b_are_false() {
        assert_eq!(Nand::nand(false, false), true)
    }

    #[test]
    fn test_nand_a_is_true_and_b_is_false() {
        assert_eq!(Nand::nand(true, false), true)
    }

    #[test]
    fn test_nand_a_is_false_and_b_is_true() {
        assert_eq!(Nand::nand(false, true), true)
    }

    #[test]
    fn test_nand_a_and_b_are_both_true() {
        assert_eq!(Nand::nand(true, true), false)
    }
}