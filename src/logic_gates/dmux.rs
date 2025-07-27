/**
 * Demultiplexor:
 * [a, b] = [in, 0] if sel = 0
 *          [0, in] if sel = 1
 */
struct Dmux;

impl Dmux {
    fn dmux(input: bool, select: bool) -> (bool, bool) {
        match select {
            false => (input, false),
            true  => (false, input)
        }
    }
}