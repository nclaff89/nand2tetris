use crate::logic_gates::nand::Nand;

/**
 * Not gate:
 * if (in) out = 0, else out = 1
 */

pub struct Not;

impl Not {
    pub fn not(a: bool) -> bool {
        Nand::nand(a, a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_false() {
        assert_eq!(Not::not(false), true);
    }
    #[test]
    fn test_not_true() {
        assert_eq!(Not::not(true), false);
    }
}