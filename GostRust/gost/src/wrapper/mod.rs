#![allow(dead_code)]

mod gen;

pub const SHIFT: u32 = gen::SHIFT;
pub const MINBLOCKLEN: u32 = gen::MINBLOCKLEN;
pub const MAXBLOCKLEN: u32 = gen::MAXBLOCKLEN;
pub const MINKEYLEN: u32 = gen::MINKEYLEN;
pub const MAXKEYLEN: u32 = gen::MAXKEYLEN;
pub const KEYLENSTEP: u32 = gen::KEYLENSTEP;

pub fn init(xi: u32) -> () {
    unsafe {
        gen::shim_init(xi);
    }
}

pub fn rnext() -> u8 {
    unsafe { gen::shim_rnext() }
}

pub fn linOp(din: &[u8], dout: &mut [u8], blen: i32) -> () {
    unsafe {
        gen::shim_linOp(
            din.as_ptr() as *mut cty::c_void,
            dout.as_mut_ptr() as *mut cty::c_void,
            blen,
        );
    }
}
