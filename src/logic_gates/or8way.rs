use crate::logic_gates::or::Or;

/**
 * 8-way Or gate:
 * out = in[0] Or in[1] Or ... Or in[7]
 */
pub struct Or8Way;

impl Or8Way {
    pub fn or8way(input: [bool; 8]) -> bool {
        let or_out_1 = Or::or(input[0], input[1]);
        let or_out_2 = Or::or(or_out_1, input[2]);
        let or_out_3 = Or::or(or_out_2, input[3]);
        let or_out_4 = Or::or(or_out_3, input[4]);
        let or_out_5 = Or::or(or_out_4, input[5]);
        let or_out_6 = Or::or(or_out_5, input[6]);
        Or::or(or_out_6, input[7])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_or8way_all_false() {
        let input = [false; 8];
        assert_eq!(Or8Way::or8way(input), false);
    }

    #[test]
    fn test_or8way_all_true() {
        let input = [true; 8];
        assert_eq!(Or8Way::or8way(input), true);
    }

    #[test]
    fn test_or8way_single_true() {
        for i in 0..8 {
            let mut input = [false; 8];
            input[i] = true;
            assert_eq!(Or8Way::or8way(input), true, "Failed with true at index {}", i);
        }
    }

    #[test]
    fn test_or8way_first_half_true() {
        let input = [true, true, true, true, false, false, false, false];
        assert_eq!(Or8Way::or8way(input), true);
    }

    #[test]
    fn test_or8way_last_half_true() {
        let input = [false, false, false, false, true, true, true, true];
        assert_eq!(Or8Way::or8way(input), true);
    }

    #[test]
    fn test_or8way_alternating() {
        let input = [true, false, true, false, true, false, true, false];
        assert_eq!(Or8Way::or8way(input), true);
    }
}
