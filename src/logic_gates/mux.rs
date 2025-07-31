use crate::logic_gates::and::And;
use crate::logic_gates::not::Not;
use crate::logic_gates::or::Or;

/**
 * Multiplexor:
 * if (sel = 0) out = a else out = b
 */
pub struct Mux;

impl Mux {
    pub fn mux(a: bool, b: bool, select: bool) -> bool {
        let not_select = Not::not(select);
        let a_and_not_select = And::and(a, not_select);
        let b_and_select = And::and(b, select);
        Or::or(a_and_not_select, b_and_select)
    }
}

#[cfg(test)]
mod test_mux {
    use super::*;

    #[test]
    fn test_mux() {
        assert_eq!(Mux::mux(false, false, false), false);
        assert_eq!(Mux::mux(false, false, true), false);
        assert_eq!(Mux::mux(true, true, false), true);
        assert_eq!(Mux::mux(true, true, true), true);
        assert_eq!(Mux::mux(false, true, false), false);
        assert_eq!(Mux::mux(false, true, true), true);
        assert_eq!(Mux::mux(true, false, false), true);
        assert_eq!(Mux::mux(true, false, true), false);
    }
}