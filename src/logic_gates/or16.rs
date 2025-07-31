use crate::logic_gates::or::Or;

/**
 * 16-bit Or gate:
 * for i = 0, ..., 15:
 * out[i] = a[i] Or b[i]
 */
pub struct Or16;

impl Or16 {
    fn or16(a: [bool; 16], b: [bool; 16]) -> [bool; 16] {
        [
            Or::or(a[0], b[0]),
            Or::or(a[1], b[1]),
            Or::or(a[2], b[2]),
            Or::or(a[3], b[3]),
            Or::or(a[4], b[4]),
            Or::or(a[5], b[5]),
            Or::or(a[6], b[6]),
            Or::or(a[7], b[7]),
            Or::or(a[8], b[8]),
            Or::or(a[9], b[9]),
            Or::or(a[10], b[10]),
            Or::or(a[11], b[11]),
            Or::or(a[12], b[12]),
            Or::or(a[13], b[13]),
            Or::or(a[14], b[14]),
            Or::or(a[15], b[15]),
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
        assert_eq!(Or16::or16(a, b), expected);
    }

    #[test]
    fn test_all_true_inputs() {
        let a = [true; 16];
        let b = [true; 16];
        let expected = [true; 16];
        assert_eq!(Or16::or16(a, b), expected);
    }

    #[test]
    fn test_a_all_true_b_all_false() {
        let a = [true; 16];
        let b = [false; 16];
        let expected = [true; 16];
        assert_eq!(Or16::or16(a, b), expected);
    }

    #[test]
    fn test_a_all_false_b_all_true() {
        let a = [false; 16];
        let b = [true; 16];
        let expected = [true; 16];
        assert_eq!(Or16::or16(a, b), expected);
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
        let expected = [true; 16]; // every bit has one true
        assert_eq!(Or16::or16(a, b), expected);
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
            true, true, true, false,
            true, true, true, false,
            true, true, false, true,
            true, true, true, true,
        ];
        assert_eq!(Or16::or16(a, b), expected);
    }
}


