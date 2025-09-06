use crate::chips::add16::Add16;

pub struct Inc16;

impl Inc16 {
    pub fn inc16(input: [bool;16]) -> [bool;16] {
        let one = [false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, true];
        Add16::add16(input, one)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inc16_one_equals_two() {
        // 1+1=2
        // 0000 0000 0000 0001
        let input = [false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, true];

        // 0000 0000 0000 0010
        let expected = [false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, true, false];

        assert_eq!(Inc16::inc16(input), expected);
    }

    #[test]
    fn test_inc16_overflow() {
        let input = [true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true];

        let expected = [false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false];

        assert_eq!(Inc16::inc16(input), expected);
    }

    #[test]
    fn test_inc16_zero_equals_one() {
        // 0 + 1 = 1
        let input = [false; 16];

        let expected = [false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, true];

        assert_eq!(Inc16::inc16(input), expected);
    }

    #[test]
    fn test_inc16_42_equals_43() {
        // 42 + 1 = 43
        // 0000 0000 0010 1010
        let input = [
            false, false, false, false,
            false, false, false, false,
            false, false, true, false,
            true, false, true, false
        ];

        // 0000 0000 0010 1011
        let expected = [
            false, false, false, false,
            false, false, false, false,
            false, false, true, false,
            true, false, true, true
        ];

        assert_eq!(Inc16::inc16(input), expected);
    }

    #[test]
    fn test_inc16_255_equals_256() {
        // 255
        // 0000 0000 1111 1111
        let input = [
            false, false, false, false,
            false, false, false, false,
            true, true, true, true,
            true, true, true, true
        ];

        // 256
        // 0000 0001 0000 0000
        let expected = [
            false, false, false, false,
            false, false, false, true,
            false, false, false, false,
            false, false, false, false
        ];

        assert_eq!(Inc16::inc16(input), expected);
    }
    
}
