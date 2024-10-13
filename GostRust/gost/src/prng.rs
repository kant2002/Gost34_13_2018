
static mut X : u32 = 1;

pub fn init(ix: u32) {
    unsafe {
        X = ix;
    }
}

/// ISO/IEC 9899:201x p347 (http://www.open-std.org/jtc1/sc22/wg14/www/docs/n1570.pdf)
/// return (x = (1103515245 * x + 12345)) >> 16;
pub fn rnext() -> u8 {
    unsafe {
        X = X * 1103515245 + 12345;
        let r = (X / 65536) % 32768;
        r as u8
    }
}