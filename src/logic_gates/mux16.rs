use crate::logic_gates::mux::Mux;

/**
 * 16-bit multiplexor:
 * for i = 0, ..., 15:
 * if (sel = 0) out[i] = a[i], else out[i] = b[i]
 */
pub struct Mux16;

impl Mux16 {
    pub fn mux16(a: [bool; 16], b: [bool; 16], select: bool) -> [bool; 16] {
        [
            Mux::mux(a[0], b[0], select),
            Mux::mux(a[1], b[1], select),
            Mux::mux(a[2], b[2], select),
            Mux::mux(a[3], b[3], select),
            Mux::mux(a[4], b[4], select),
            Mux::mux(a[5], b[5], select),
            Mux::mux(a[6], b[6], select),
            Mux::mux(a[7], b[7], select),
            Mux::mux(a[8], b[8], select),
            Mux::mux(a[9], b[9], select),
            Mux::mux(a[10], b[10], select),
            Mux::mux(a[11], b[11], select),
            Mux::mux(a[12], b[12], select),
            Mux::mux(a[13], b[13], select),
            Mux::mux(a[14], b[14], select),
            Mux::mux(a[15], b[15], select),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mux16_selects_a_when_false() {
        let a = [true; 16];
        let b = [false; 16];
        let select = false;
        let result = Mux16::mux16(a, b, select);
        assert_eq!(result, a);
    }

    #[test]
    fn test_mux16_selects_b_when_true() {
        let a = [false; 16];
        let b = [true; 16];
        let select = true;
        let result = Mux16::mux16(a, b, select);
        assert_eq!(result, b);
    }

    #[test]
    fn test_mux16_mixed_values() {
        let a = [
            true, false, true, false, true, false, true, false,
            true, false, true, false, true, false, true, false,
        ];
        let b = [
            false, true, false, true, false, true, false, true,
            false, true, false, true, false, true, false, true,
        ];

        let result_a = Mux16::mux16(a, b, false);
        let result_b = Mux16::mux16(a, b, true);

        assert_eq!(result_a, a);
        assert_eq!(result_b, b);
    }

    #[test]
    fn test_mux16_all_false() {
        let a = [false; 16];
        let b = [false; 16];
        assert_eq!(Mux16::mux16(a, b, false), [false; 16]);
        assert_eq!(Mux16::mux16(a, b, true), [false; 16]);
    }

    #[test]
    fn test_mux16_all_true() {
        let a = [true; 16];
        let b = [true; 16];
        assert_eq!(Mux16::mux16(a, b, false), [true; 16]);
        assert_eq!(Mux16::mux16(a, b, true), [true; 16]);
    }
}
