/**
 * Or gate:
 * if (a or b) out = 1, else out = 0
 */

pub struct Or;

impl Or {
    pub fn or(a: bool, b: bool) -> bool {
        a || b
    }
}

#[cfg(test)]
mod test_or {
    use super::*;

    #[test]
    fn test_or() {
        assert_eq!(Or::or(true, false), true);
        assert_eq!(Or::or(true, true), true);
        assert_eq!(Or::or(false, true), true);
        assert_eq!(Or::or(false, false), false);
    }
}