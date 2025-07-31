use crate::logic_gates::and::And;
use crate::logic_gates::not::Not;

/**
 * Demultiplexor:
 * [a, b] = [in, 0] if sel = 0
 *          [0, in] if sel = 1
 */
pub struct Dmux;

impl Dmux {
    pub(crate) fn dmux(input: bool, select: bool) -> (bool, bool) {
        let not_select = Not::not(select);
        (And::and(input, not_select), And::and(input, select))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dmux() {
       assert_eq!(Dmux::dmux(true, true), (false, true));
       assert_eq!(Dmux::dmux(true, false), (true, false));
       assert_eq!(Dmux::dmux(false, false), (false, false));
       assert_eq!(Dmux::dmux(false, true), (false, false));
    }
}