/**
 * Multiplexor:
 * if (sel = 0) out = a else out = b
 */
pub struct Mux;

impl Mux {
    pub fn mux(a: bool, b: bool, select: bool) -> bool {
        match select {
            true => b,
            false => a,
        }
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