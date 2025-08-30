use crate::chips::full_adder::FullAdder;
use crate::chips::half_adder::HalfAdder;

pub struct Add16 {

}

impl Add16 {
    pub fn add16(a:[bool; 16], b: [bool; 16]) -> [bool; 16] {
        let mut sum = [false; 16];

        let (sum15, carry15) = HalfAdder::half_adder(a[15], b[15]);
        sum[15] = sum15;

        let (sum14, carry14) = FullAdder::full_adder(a[14], b[14], carry15);
        sum[14] = sum14;

        let (sum13, carry13) = FullAdder::full_adder(a[13], b[13], carry14);
        sum[13] = sum13;

        let (sum12, carry12) = FullAdder::full_adder(a[12], b[12], carry13);
        sum[12] = sum12;

        let (sum11, carry11) = FullAdder::full_adder(a[11], b[11], carry12);
        sum[11] = sum11;

        let (sum10, carry10) = FullAdder::full_adder(a[10], b[10], carry11);
        sum[10] = sum10;

        let (sum9, carry9) = FullAdder::full_adder(a[9], b[9], carry10);
        sum[9] = sum9;

        let (sum8, carry8) = FullAdder::full_adder(a[8], b[8], carry9);
        sum[8] = sum8;

        let (sum7, carry7) = FullAdder::full_adder(a[7], b[7], carry8);
        sum[7] = sum7;

        let (sum6, carry6) = FullAdder::full_adder(a[6], b[6], carry7);
        sum[6] = sum6;

        let (sum5, carry5) = FullAdder::full_adder(a[5], b[5], carry6);
        sum[5] = sum5;

        let (sum4, carry4) = FullAdder::full_adder(a[4], b[4], carry5);
        sum[4] = sum4;

        let (sum3, carry3) = FullAdder::full_adder(a[3], b[3], carry4);
        sum[3] = sum3;

        let (sum2, carry2) = FullAdder::full_adder(a[2], b[2], carry3);
        sum[2] = sum2;

        let (sum1, carry1) = FullAdder::full_adder(a[1], b[1], carry2);
        sum[1] = sum1;

        let sum0= FullAdder::full_adder(a[0], b[0], carry1);
        sum[0] = sum0.0;


        sum
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_add16_all_true() {
        // 1111111111111110
        let expected = [true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, false];
        assert_eq!(Add16::add16([true;16], [true; 16]), expected);
    }

    #[test]
    fn test_add16_all_false() {
        // 0000000000000000
        assert_eq!(Add16::add16([false;16], [false;16]), [false; 16]);
    }

    #[test]
    fn test_add16_a_b_alternate() {
        let a = [true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false];
        let b = [false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true];

        let expected = [true; 16];

        assert_eq!(Add16::add16(a, b), expected);

    }

    #[test]
    fn test_add16_1_plus_1_is_2() {
        let a = [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true];
        let b = [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true];

        let expected = [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false];

        assert_eq!(Add16::add16(a, b), expected);
    }

    #[test]
    fn test_add16_2_plus_2_is_4() {
        let a =  [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false];
        let b = [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false];

        let expected = [false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false];

        assert_eq!(Add16::add16(a, b), expected);
    }

    #[test]
    fn test_add16_max_plus_one_wraps() {
        // 0xFFFF + 1 = 0x0000 (carry out ignored)
        let a = [true; 16];
        let mut b = [false; 16];
        b[15] = true; // least significant bit = 1

        let expected = [false; 16];
        assert_eq!(Add16::add16(a, b), expected);
    }

    #[test]
    fn test_add16_carry_propagation_middle() {
        // 0x00FF + 0x0001 = 0x0100
        let mut a = [false; 16];
        for i in 8..16 {
            a[i] = true; // lower 8 bits = 11111111
        }

        let mut b = [false; 16];
        b[15] = true; // +1

        let mut expected = [false; 16];
        expected[7] = true; // result = 0000000100000000

        assert_eq!(Add16::add16(a, b), expected);
    }

    #[test]
    fn test_add16_random_case() {
        let a = [
            true, false, true, false,  true, false, true, false,
            false, false, false, false,  true, true, true, true
        ];
        let b = [
            false, true, false, true,  false, true, false, true,
            true, true, true, true,  false, false, false, true
        ];

        let expected = [false; 16];

        assert_eq!(Add16::add16(a, b), expected);
    }

    #[test]
    fn test_add16_one_plus_max_minus_one() {
        // (0x0001 + 0xFFFE) = 0xFFFF
        let mut a = [false; 16];
        a[15] = true;

        let mut b = [true; 16];
        b[15] = false;

        let expected = [true; 16];
        assert_eq!(Add16::add16(a, b), expected);
    }

}