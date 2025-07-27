/**
 * 16-bit And gate:
 * for i = 0, ..., 15:
 * out[i] = a[i] And b[i]
 */
struct And16;

impl And16 {
    fn and16(a: [bool; 16], b: [bool; 16]) -> [bool; 16] {
        let mut output = [false; 16];

        for i in 0..16 {
            output[i] = a[i] && b[i];
        }

        output
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
