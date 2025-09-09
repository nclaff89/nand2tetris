use crate::logic_gates::and::And;

/**
 * 16-bit And gate:
 * for i = 0, ..., 15:
 * out[i] = a[i] And b[i]
 */
pub struct And16;

impl And16 {
    pub fn and16(a: [bool; 16], b: [bool; 16]) -> [bool; 16] {
        [
            And::and(a[0], b[0]),
            And::and(a[1], b[1]),
            And::and(a[2], b[2]),
            And::and(a[3], b[3]),
            And::and(a[4], b[4]),
            And::and(a[5], b[5]),
            And::and(a[6], b[6]),
            And::and(a[7], b[7]),
            And::and(a[8], b[8]),
            And::and(a[9], b[9]),
            And::and(a[10], b[10]),
            And::and(a[11], b[11]),
            And::and(a[12], b[12]),
            And::and(a[13], b[13]),
            And::and(a[14], b[14]),
            And::and(a[15], b[15]),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_false_inputs() {
        let a = [false; 16];
        let b = [false; 16];
        let expected = [false; 16];
        assert_eq!(And16::and16(a, b), expected);
    }

    #[test]
    fn test_all_true_inputs() {
        let a = [true; 16];
        let b = [true; 16];
        let expected = [true; 16];
        assert_eq!(And16::and16(a, b), expected);
    }

    #[test]
    fn test_a_all_true_b_all_false() {
        let a = [true; 16];
        let b = [false; 16];
        let expected = [false; 16];
        assert_eq!(And16::and16(a, b), expected);
    }

    #[test]
    fn test_a_all_false_b_all_true() {
        let a = [false; 16];
        let b = [true; 16];
        let expected = [false; 16];
        assert_eq!(And16::and16(a, b), expected);
    }

    #[test]
    fn test_alternating_inputs() {
        let a = [
            true, false, true, false,
            true, false, true, false,
            true, false, true, false,
            true, false, true, false,
        ];
        let b = [
            false, true, false, true,
            false, true, false, true,
            false, true, false, true,
            false, true, false, true,
        ];
        let expected = [false; 16]; // always one false = false
        assert_eq!(And16::and16(a, b), expected);
    }

    #[test]
    fn test_mixed_pattern() {
        let a = [
            true, true, false, false,
            true, false, true, false,
            false, true, false, true,
            true, false, false, true,
        ];
        let b = [
            true, false, true, false,
            false, true, true, false,
            true, false, false, true,
            false, true, true, false,
        ];
        let expected = [
            true, false, false, false,
            false, false, true, false,
            false, false, false, true,
            false, false, false, false,
        ];
        assert_eq!(And16::and16(a, b), expected);
    }
}
