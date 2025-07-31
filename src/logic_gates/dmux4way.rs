use crate::logic_gates::dmux::Dmux;

/**
 * 4-way demultiplexor:
 * [a, b, c, d] = [in, 0, 0, 0] if sel = 00
 *                [0, in, 0, 0] if sel = 01
 *                [0, 0, in, 0] if sel = 10
 *                [0, 0, 0, in] if sel = 11
 */
pub struct Dmux4Way;
// todo, think more on this, we basically had to flip the selects around to make it match the HDL because
// of the way HDL treats least and most significant bits. for example hdl select 01 in the code below we reverse it
// implication is that all vectors of booleans used as inputs should be in reverse order to match binary.
// least significant is the rightmost in binary.

// I really don't know if i like this. but i guess we'll roll with it for now and see how it goes.
impl Dmux4Way {
    fn dmux4way(input: bool, select: [bool; 2]) -> [bool; 4] {
        let (dmux_0, dmux_1) = Dmux::dmux(input, select[1]);
        let (a, b) = Dmux::dmux(dmux_0, select[0]);
        let (c, d) = Dmux::dmux(dmux_1, select[0]);
        [a, b, c, d]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dmux4way_all_cases() {
        // input = false, all outputs should be false regardless of select
        assert_eq!(Dmux4Way::dmux4way(false, [false, false]), [false, false, false, false]);
        assert_eq!(Dmux4Way::dmux4way(false, [true,  false]), [false, false, false, false]);
        assert_eq!(Dmux4Way::dmux4way(false, [false, true ]), [false, false, false, false]);
        assert_eq!(Dmux4Way::dmux4way(false, [true,  true ]), [false, false, false, false]);

        // input = true, output routes input based on select (LSB, MSB)
        assert_eq!(Dmux4Way::dmux4way(true,  [false, false]), [true,  false, false, false]); // sel=00
        assert_eq!(Dmux4Way::dmux4way(true,  [true,  false]), [false, true,  false, false]); // sel=01
        assert_eq!(Dmux4Way::dmux4way(true,  [false, true ]), [false, false, true,  false]); // sel=10
        assert_eq!(Dmux4Way::dmux4way(true,  [true,  true ]), [false, false, false, true ]); // sel=11
    }
}
