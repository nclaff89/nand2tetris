/**
 * And gate:
 * if (a and b) out = 1, else out = 0
 */
pub struct And;

impl And {
    pub fn and(a: bool, b: bool) -> bool {
        a && b
    }
}

#[cfg(test)]
mod test_and {
    use super::*;
    #[test]
    fn test_and() {
        assert_eq!(And::and(true, true), true);
        assert_eq!(And::and(true, false), false);
        assert_eq!(And::and(false, true), false);
        assert_eq!(And::and(false, false), false);
    }
}