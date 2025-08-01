use crate::logic_gates::mux16::Mux16;
use crate::logic_gates::mux4way16::Mux4Way16;

/**
 * 8-way 16-bit multiplexor:
 * out = a if sel = 000
 *       b if sel = 001
 *       c if sel = 010
 *       d if sel = 011
 *       e if sel = 100
 *       f if sel = 101
 *       g if sel = 110
 *       h if sel = 111
 */
pub struct Mux8Way16;

impl Mux8Way16 {
    pub fn mux8way16(
        a: [bool; 16], b: [bool; 16], c: [bool; 16], d: [bool; 16],
        e: [bool; 16], f: [bool; 16], g: [bool; 16], h: [bool; 16], select:[bool; 3])
        -> [bool; 16]
    {
        let mux4way16_abcd = Mux4Way16::mux4way16(a, b, c, d, [select[1], select[2]]); // lsb
        let mux4way16_efgh = Mux4Way16::mux4way16(e, f, g, h, [select[1], select[2]]); // lsb
        Mux16::mux16(mux4way16_abcd, mux4way16_efgh, select[0]) // msb
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const a: [bool; 16] = [true, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false];

    const b: [bool; 16] = [false, true, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false];

    const c: [bool; 16] = [false, false, true, false, false, false, false, false,
            false, false, false, false, false, false, false, false];

    const d: [bool; 16] = [false, false, false, true, false, false, false, false,
            false, false, false, false, false, false, false, false];

    const e: [bool; 16] = [false, false, false, false, true, false, false, false,
            false, false, false, false, false, false, false, false];


    const f: [bool; 16] =
        [false, false, false, false, false, true, false, false,
            false, false, false, false, false, false, false, false];

    const g: [bool; 16] = [false, false, false, false, false, false, true, false,
            false, false, false, false, false, false, false, false];

    const h: [bool; 16] = [false, false, false, false, false, false, false, true,
            false, false, false, false, false, false, false, false];

    #[test]
    fn test_mux8way16_000() {
        let out = Mux8Way16::mux8way16(
            a, b, c, d,
            e, f, g, h,
            [false, false, false]
        );
        assert_eq!(out, a);
    }

    #[test]
    fn test_mux8way16_001() {
        let out = Mux8Way16::mux8way16(
            a, b, c, d,
            e, f, g, h,
            [false, false, true]
        );
        assert_eq!(out, b);
    }

    #[test]
    fn test_mux8way16_010() {
        let out = Mux8Way16::mux8way16(
            a, b, c, d,
            e, f, g, h,
            [false, true, false]
        );
        assert_eq!(out, c);
    }

    #[test]
    fn test_mux8way16_011() {
        let out = Mux8Way16::mux8way16(
            a, b, c, d,
            e, f, g, h,
            [false, true, true]
        );
        assert_eq!(out, d);
    }

    #[test]
    fn test_mux8way16_100() {
        let out = Mux8Way16::mux8way16(
            a, b, c, d,
            e, f, g, h,
            [true, false, false]
        );
        assert_eq!(out, e);
    }

    #[test]
    fn test_mux8way16_101() {
        let out = Mux8Way16::mux8way16(
            a, b, c, d,
            e, f, g, h,
            [true, false, true]
        );
        assert_eq!(out, f);
    }

    #[test]
    fn test_mux8way16_110() {
        let out = Mux8Way16::mux8way16(
            a, b, c, d,
            e, f, g, h,
            [true, true, false]
        );
        assert_eq!(out, g);
    }

    #[test]
    fn test_mux8way16_111() {
        let out = Mux8Way16::mux8way16(
            a, b, c, d,
            e, f, g, h,
            [true, true, true]
        );
        assert_eq!(out, h);
    }
}

