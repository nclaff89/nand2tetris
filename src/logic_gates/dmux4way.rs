/**
 * 4-way demultiplexor:
 * [a, b, c, d] = [in, 0, 0, 0] if sel = 00
 *                [0, in, 0, 0] if sel = 01
 *                [0, 0, in, 0] if sel = 10
 *                [0, 0, 0, in] if sel = 11
 */
pub struct Dmux4Way;

impl Dmux4Way {
    fn dmux4way(input: bool, select: [bool; 2]) -> [bool; 4] {
        if select[0] == false && select[1] == false {
            [input, false, false, false]
        } else if select[0] == false && select[1] == true {
            [false, input, false, false]
        } else if select[0] == true && select[1] == false {
            [false, false, input, false]
        } else {
            // both selectors are true
            [false, false, false, input]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dmux4way_select_00() {
        let input = true;
        let select = [false, false];
        assert_eq!(Dmux4Way::dmux4way(input, select), [true, false, false, false]);
    }

    #[test]
    fn test_dmux4way_select_01() {
        let input = true;
        let select = [false, true];
        assert_eq!(Dmux4Way::dmux4way(input, select), [false, true, false, false]);
    }

    #[test]
    fn test_dmux4way_select_10() {
        let input = true;
        let select = [true, false];
        assert_eq!(Dmux4Way::dmux4way(input, select), [false, false, true, false]);
    }

    #[test]
    fn test_dmux4way_select_11() {
        let input = true;
        let select = [true, true];
        assert_eq!(Dmux4Way::dmux4way(input, select), [false, false, false, true]);
    }

    #[test]
    fn test_dmux4way_input_false() {
        for sel in [
            [false, false],
            [false, true],
            [true, false],
            [true, true],
        ] {
            assert_eq!(Dmux4Way::dmux4way(false, sel), [false, false, false, false]);
        }
    }
}
