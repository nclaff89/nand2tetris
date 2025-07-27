/**
 * 16-bit Not gate:
 * for i = 0, ..., 15:
 * out[i] = Not(a[i])
 */
pub struct Not16;

impl Not16 {
    pub fn not16(input: [bool; 16]) -> [bool; 16] {
        let mut output: [bool; 16] = Default::default();
        for i in 0..input.len() {
            output[i] = !input[i];
        }
        output
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