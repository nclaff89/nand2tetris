use crate::logic_gates::mux16::Mux16;

/**
 * 4-way 16-bit multiplexor:
 * out = a if sel = 00
 *       b if sel = 01
 *       c if sel = 10
 *       d if sel = 11
 */
pub struct Mux4Way16;

impl Mux4Way16 {
    pub fn mux4way16(a: [bool; 16], b: [bool; 16], c: [bool; 16], d: [bool; 16], select: [bool; 2]) -> [bool; 16] {
        let mux_16_ab = Mux16::mux16(a, b, select[1]); // least significant bit
        let mux_16_cd = Mux16::mux16(c, d, select[1]); // least significant bit
        Mux16::mux16(mux_16_ab, mux_16_cd, select[0]) // most significant bit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mux16_00(){
        let a = [
            true, false, true, false, true, true, false, false,
            false, false, false, false, true, true, false, true
        ];

        let b = [
            false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false
        ];

        let c = [
            true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true
        ];

        let d = [
            true, false, true, false, true, false, true, false, true,
            false, true, false, true, false, true, false
        ];

        let select = [false, false];

        assert_eq!(Mux4Way16::mux4way16(a, b, c, d, select), a);
    }

    #[test]
    fn test_mux16_01(){
        let a = [
            true, false, true, false, true, true, false, false,
            false, false, false, false, true, true, false, true
        ];

        let b = [
            false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false
        ];

        let c = [
            true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true
        ];

        let d = [
            true, false, true, false, true, false, true, false, true,
            false, true, false, true, false, true, false
        ];

        let select = [false, true];

        assert_eq!(Mux4Way16::mux4way16(a, b, c, d, select), b);
    }

    #[test]
    fn test_mux16_10(){
        let a = [
            true, false, true, false, true, true, false, false,
            false, false, false, false, true, true, false, true
        ];

        let b = [
            false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false
        ];

        let c = [
            true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true
        ];

        let d = [
            true, false, true, false, true, false, true, false, true,
            false, true, false, true, false, true, false
        ];

        let select = [true, false];

        assert_eq!(Mux4Way16::mux4way16(a, b, c, d, select), c);
    }

    #[test]
    fn test_mux16_11() {
        let a = [
            true, false, true, false, true, true, false, false,
            false, false, false, false, true, true, false, true
        ];

        let b = [
            false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false
        ];

        let c = [
            true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true
        ];

        let d = [
            true, false, true, false, true, false, true, false, true,
            false, true, false, true, false, true, false
        ];

        let select = [true, true];

        assert_eq!(Mux4Way16::mux4way16(a, b, c, d, select), d);
    }
}
