#![allow(dead_code)]

mod gen;

pub fn cc_init(xi: u32) -> () {
    unsafe {
        gen::shim_init(xi);
    }
}

pub fn cc_rnext() -> u8 {
    unsafe { gen::shim_rnext() }
}

#[allow(non_snake_case)]
pub fn cc_linOp(din: &[u8], dout: &mut [u8], blen: i32) -> () {
    unsafe {
        gen::shim_linOp(
            din.as_ptr() as *mut cty::c_void,
            dout.as_mut_ptr() as *mut cty::c_void,
            blen,
        );
    }
}

pub fn cc_Kexp(key: &[u8], klen: i32, blen:i32 , rkey: &mut [u8]) -> () {
    unsafe {
        gen::shim_Kexp(
            key.as_ptr() as *mut u8,
            klen,
            blen,
            rkey.as_mut_ptr(),
        );
    }
}
