pub const SHIFT: u32 = 17;
pub const MINBLOCKLEN: u32 = 16;
pub const MAXBLOCKLEN: u32 = 64;
pub const MINKEYLEN: u32 = 32;
pub const MAXKEYLEN: u32 = 128;
pub const KEYLENSTEP: u32 = 16;

#[allow(non_snake_case)]
#[inline]
pub fn RNDS(x: i32) -> i32 {
    16 + (x - 32) / 16
}

#[allow(non_snake_case)]
#[inline]
pub fn ROTL(x: u32, s: u32) -> u32 {
    (x << s) | (x >> (32 - s))
}

#[allow(non_snake_case)]
#[inline]
pub fn ROTL64(x: u64, s: u64) -> u64 {
    (x << s) | (x >> (32 - s))
}

#[allow(non_snake_case)]
fn Kexp(key: &[u8], klen: u32, blen: u32, rkey: &[u8]) {
    let mut r0: [u8; 17] = [0; 17];
    let mut r1: [u8; 15] = [0; 15];
    let addk = klen - 32;
    let step = 0;
    let s = SHIFT;
    r0.iter_mut()
        .take(15)
        .enumerate()
        .for_each(|x| *x.1 = key[2 * x.0]);
    r1.iter_mut()
        .take(15)
        .enumerate()
        .for_each(|x| *x.1 = key[2 * x.0 + 1]);
    r0[15] = key[30];
    r0[16] = key[31];
}
