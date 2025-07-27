/**
 * Demultiplexor:
 * [a, b] = [in, 0] if sel = 0
 *          [0, in] if sel = 1
 */
struct Dmux;

impl Dmux {
    fn dmux(input: bool, select: bool) -> (bool, bool) {
        match select {
            false => (input, false),
            true  => (false, input)
        }
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