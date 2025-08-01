use crate::logic_gates::dmux4way::Dmux4Way;
use crate::logic_gates::dmux::Dmux;

/**
 * 8-way demultiplexor:
 * [a, b, c, d, e, f, g, h] = [in, 0,  0,  0,  0,  0,  0,  0] if sel = 000
 *                            [0, in,  0,  0,  0,  0,  0,  0] if sel = 001
 *                            [0,  0, in,  0,  0,  0,  0,  0] if sel = 010
 *                            [0,  0,  0, in,  0,  0,  0,  0] if sel = 011
 *                            [0,  0,  0,  0, in,  0,  0,  0] if sel = 100
 *                            [0,  0,  0,  0,  0, in,  0,  0] if sel = 101
 *                            [0,  0,  0,  0,  0,  0, in,  0] if sel = 110
 *                            [0,  0,  0,  0,  0,  0,  0, in] if sel = 111
 */
pub struct Dmux8Way;

impl Dmux8Way {
    pub fn dmux8way(input: bool, select: [bool; 3]) -> [bool; 8]{
        let dmux4way = Dmux4Way::dmux4way(input, [select[0], select[1]]); //msb
        let (a,b) = Dmux::dmux(dmux4way[0], select[2]); //lsb
        let (c, d) = Dmux::dmux(dmux4way[1], select[2]); //lsb
        let (e, f) = Dmux::dmux(dmux4way[2], select[2]); //lsb
        let (g, h) = Dmux::dmux(dmux4way[3], select[2]); //lsb
        [a, b, c, d, e, f, g, h]

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dmux8way_all_cases() {
        // input = false, all outputs should be false regardless of select
        assert_eq!(Dmux8Way::dmux8way(false, [false, false, false]), [false, false, false, false, false, false, false, false]);
        assert_eq!(Dmux8Way::dmux8way(false, [false, false, true ]), [false, false, false, false, false, false, false, false]);
        assert_eq!(Dmux8Way::dmux8way(false, [false, true,  false]), [false, false, false, false, false, false, false, false]);
        assert_eq!(Dmux8Way::dmux8way(false, [false, true,  true ]), [false, false, false, false, false, false, false, false]);
        assert_eq!(Dmux8Way::dmux8way(false, [true,  false, false]), [false, false, false, false, false, false, false, false]);
        assert_eq!(Dmux8Way::dmux8way(false, [true,  false, true ]), [false, false, false, false, false, false, false, false]);
        assert_eq!(Dmux8Way::dmux8way(false, [true,  true,  false]), [false, false, false, false, false, false, false, false]);
        assert_eq!(Dmux8Way::dmux8way(false, [true,  true,  true ]), [false, false, false, false, false, false, false, false]);

        // input = true, output routes input based on select (MSB to LSB: [s2, s1, s0])
        assert_eq!(Dmux8Way::dmux8way(true, [false, false, false]), [true,  false, false, false, false, false, false, false]); // sel=000
        assert_eq!(Dmux8Way::dmux8way(true, [false, false, true ]), [false, true,  false, false, false, false, false, false]); // sel=001
        assert_eq!(Dmux8Way::dmux8way(true, [false, true,  false]), [false, false, true,  false, false, false, false, false]); // sel=010
        assert_eq!(Dmux8Way::dmux8way(true, [false, true,  true ]), [false, false, false, true,  false, false, false, false]); // sel=011
        assert_eq!(Dmux8Way::dmux8way(true, [true,  false, false]), [false, false, false, false, true,  false, false, false]); // sel=100
        assert_eq!(Dmux8Way::dmux8way(true, [true,  false, true ]), [false, false, false, false, false, true,  false, false]); // sel=101
        assert_eq!(Dmux8Way::dmux8way(true, [true,  true,  false]), [false, false, false, false, false, false, true,  false]); // sel=110
        assert_eq!(Dmux8Way::dmux8way(true, [true,  true,  true ]), [false, false, false, false, false, false, false, true ]); // sel=111
    }
}
