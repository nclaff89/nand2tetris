use crate::logic_gates::not::Not;

/**
 * 16-bit Not gate:
 * for i = 0, ..., 15:
 * out[i] = Not(a[i])
 */
pub struct Not16;

impl Not16 {
    pub fn not16(input: [bool; 16]) -> [bool; 16] {
        [
            Not::not(input[0]),
            Not::not(input[1]),
            Not::not(input[2]),
            Not::not(input[3]),
            Not::not(input[4]),
            Not::not(input[5]),
            Not::not(input[6]),
            Not::not(input[7]),
            Not::not(input[8]),
            Not::not(input[9]),
            Not::not(input[10]),
            Not::not(input[11]),
            Not::not(input[12]),
            Not::not(input[13]),
            Not::not(input[14]),
            Not::not(input[15]),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_false() {
        let input = [false; 16];
        let expected = [true; 16];
        let result = Not16::not16(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_all_true() {
        let input = [true; 16];
        let expected = [false; 16];
        let result = Not16::not16(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_alternating_bits() {
        let input = [
            true, false, true, false,
            true, false, true, false,
            true, false, true, false,
            true, false, true, false,
        ];
        let expected = [
            false, true, false, true,
            false, true, false, true,
            false, true, false, true,
            false, true, false, true,
        ];
        let result = Not16::not16(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_random_pattern() {
        let input = [
            true, true, false, false,
            true, false, true, false,
            false, true, false, true,
            true, false, false, true,
        ];
        let expected = [
            false, false, true, true,
            false, true, false, true,
            true, false, true, false,
            false, true, true, false,
        ];
        let result = Not16::not16(input);
        assert_eq!(result, expected);
    }
}