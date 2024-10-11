#![allow(dead_code)]
#![allow(non_snake_case)]

mod gen;

pub fn cc_init(xi: u32) -> () {
    unsafe {
        gen::shim_init(xi);
    }
}

pub fn cc_rnext() -> u8 {
    unsafe { gen::shim_rnext() }
}

pub fn cc_linOp(din: &[u8], dout: &mut [u8], blen: i32) -> () {
    unsafe {
        gen::shim_linOp(
            din.as_ptr() as *mut cty::c_void,
            dout.as_mut_ptr() as *mut cty::c_void,
            blen,
        );
    }
}

pub fn cc_sBox(din: &[u8], dout: &mut [u8], blen: i32) -> () {
    unsafe {
        gen::shim_sBox(din.as_ptr() as *mut u8, dout.as_mut_ptr(), blen);
    }
}

pub fn cc_AddRkX(block: &[u8], rkey: &[u8], nr: i32, blen: i32, res: &mut [u8]) -> () {
    unsafe {
        gen::shim_AddRkX(
            block.as_ptr() as *mut u8,
            rkey.as_ptr() as *mut u8,
            nr,
            blen,
            res.as_mut_ptr(),
        );
    }
}

pub fn cc_AddRk(block: &[u8], rkey: &[u8], nr: i32, blen: i32, res: &mut [u8]) -> () {
    unsafe {
        gen::shim_AddRk(
            block.as_ptr() as *mut u8,
            rkey.as_ptr() as *mut u8,
            nr,
            blen,
            res.as_mut_ptr(),
        );
    }
}

pub fn cc_encrypt(block: &[u8], rkey: &[u8], klen: i32, blen: i32, res: &mut [u8]) -> () {
    unsafe {
        gen::shim_encrypt(
            block.as_ptr() as *mut u8,
            rkey.as_ptr() as *mut u8,
            klen,
            blen,
            res.as_mut_ptr(),
        );
    }
}

pub fn cc_decrypt(block: &[u8], rkey: &[u8], klen: i32, blen: i32, res: &mut [u8]) -> () {
    unsafe {
        gen::shim_decrypt(
            block.as_ptr() as *mut u8,
            rkey.as_ptr() as *mut u8,
            klen,
            blen,
            res.as_mut_ptr(),
        );
    }
}

pub fn cc_Kexp(key: &[u8], klen: i32, blen: i32, rkey: &mut [u8]) -> () {
    unsafe {
        gen::shim_Kexp(key.as_ptr() as *mut u8, klen, blen, rkey.as_mut_ptr());
    }
}
