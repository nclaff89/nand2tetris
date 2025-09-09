use crate::chips::add16::Add16;
use crate::logic_gates::and16::And16;
use crate::logic_gates::and::And;
use crate::logic_gates::mux16::Mux16;
use crate::logic_gates::not16::Not16;
use crate::logic_gates::not::Not;
use crate::logic_gates::or8way::Or8Way;
use crate::logic_gates::or::Or;

/**
 * ALU (Arithmetic Logic Unit):
 * Computes out = one of the following functions:
 *                0, 1, -1,
 *                x, y, !x, !y, -x, -y,
 *                x + 1, y + 1, x - 1, y - 1,
 *                x + y, x - y, y - x,
 *                x & y, x | y
 * on the 16-bit inputs x, y,
 * according to the input bits zx, nx, zy, ny, f, no.
 * In addition, computes the two output bits:
 * if (out == 0) zr = 1, else zr = 0
 * if (out < 0)  ng = 1, else ng = 0
 */
// Implementation: Manipulates the x and y inputs
// and operates on the resulting values, as follows:
// if (zx == 1) sets x = 0        // 16-bit constant
// if (nx == 1) sets x = !x       // bitwise not
// if (zy == 1) sets y = 0        // 16-bit constant
// if (ny == 1) sets y = !y       // bitwise not
// if (f == 1)  sets out = x + y  // integer 2's complement addition
// if (f == 0)  sets out = x & y  // bitwise and
// if (no == 1) sets out = !out   // bitwise not

pub struct ALU;

impl ALU {
    pub fn alu(
        x: [bool; 16],
        y: [bool; 16],
        zx: bool,
        nx: bool,
        zy: bool,
        ny: bool,
        f: bool,
        no: bool,
    ) -> ALUOutput {

        // zero the x input
        let x1 = Mux16::mux16(x, [false; 16], zx);

        // negate the x input?
        let x2 = Mux16::mux16(x1, Not16::not16(x1), nx);

        // zero the y input?
        let y1 = Mux16::mux16(y, [false; 16], zy);

        // negate the y input?
        let y2 = Mux16::mux16(y1, Not16::not16(y1), ny);

        // compute (out = x + y) or (out = x & y)?
        let x_plus_y = Add16::add16(x2, y2);
        let x_and_y = And16::and16(x2, y2);
        let fxy = Mux16::mux16(x_and_y, x_plus_y, f);

        // negate the output?
        let not_fxy = Not16::not16(fxy);
        let out = Mux16::mux16(fxy, not_fxy, no);

        // if (out == 0) zr = 1, else zr = 0
        let mut first_half = [false; 8];
        first_half.copy_from_slice(&out[0..8]);
        let or_8way_1 = Or8Way::or8way(first_half);

        let mut second_half = [false; 8];
        second_half.copy_from_slice(&out[8..16]);
        let or_8way_2 = Or8Way::or8way(second_half);

        let zr = Not::not(Or::or(or_8way_1, or_8way_2));

        // if (out < 0)  equals 1, else 0
        // out[15] is the most significant bit.
        let ng = And::and(out[15], true);

        ALUOutput { out, zr, ng }
    }
}

pub struct ALUOutput {
    pub out: [bool; 16],
    pub zr: bool,
    pub ng: bool,
}