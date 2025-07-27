/**
 * Exclusive-or gate:
 * if ((a and Not(b)) or (Not(a) and b)) out = 1, else out = 0
 */
pub struct Xor;

impl Xor {
    pub fn xor(a: bool, b: bool) -> bool {
        (!a && b) || (a && !b)
    }
}

#[cfg(test)]
mod tests_xor {
    use super::*;

    #[test]
    fn test_xor() {
        assert_eq!(Xor::xor(false, false), false);
        assert_eq!(Xor::xor(true, false), true);
        assert_eq!(Xor::xor(true, true), false);
        assert_eq!(Xor::xor(false, true), true);
    }
}