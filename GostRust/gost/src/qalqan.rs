#![allow(non_snake_case)]
#![allow(dead_code)]

pub const SHIFT: i32 = 17;
pub const MINBLOCKLEN: i32 = 16;
pub const MAXBLOCKLEN: i32 = 64;
pub const MINKEYLEN: i32 = 32;
pub const MAXKEYLEN: i32 = 128;
pub const KEYLENSTEP: i32 = 16;

pub const SB: [u8; 256] = [
    /* ded: OK, dif: 4, dip: 7, lin: 32, pow: 7, cor: 0, dst: 112, sac: 116..140, cyc: 256 */
    0xd1, 0xb5, 0xa6, 0x74, 0x2f, 0xb2, 0x03, 0x77, 0xae, 0xb3, 0x60, 0x95, 0xfd, 0xf8, 0xc7, 0xf0,
    0x2b, 0xce, 0xa5, 0x91, 0x4c, 0x6f, 0xf3, 0x4f, 0x82, 0x01, 0x45, 0x76, 0x9f, 0xed, 0x41, 0xfb,
    0xac, 0x4e, 0x5e, 0x04, 0xeb, 0xf9, 0xf1, 0x3a, 0x1f, 0xe2, 0x8e, 0xe7, 0x85, 0x35, 0xdb, 0x52,
    0x78, 0xa1, 0xfc, 0xa2, 0xde, 0x68, 0x02, 0x4d, 0xf6, 0xdd, 0xcf, 0xa3, 0xdc, 0x6b, 0x81, 0x44,
    0x2a, 0x5d, 0x1e, 0xe0, 0x53, 0x71, 0x3b, 0xc1, 0xcc, 0x9d, 0x80, 0xd5, 0x84, 0x00, 0x24, 0x4b,
    0xb6, 0x83, 0x0d, 0x87, 0x7e, 0x86, 0xca, 0x96, 0xbe, 0x5a, 0xe6, 0xd0, 0xd4, 0xd8, 0x55, 0xc0,
    0x05, 0xe5, 0xe9, 0x5b, 0x47, 0xe4, 0x2d, 0x34, 0x13, 0x88, 0x48, 0x32, 0x38, 0xb9, 0xda, 0xc9,
    0x42, 0x29, 0xd7, 0xf2, 0x9b, 0x6d, 0xe8, 0x8d, 0x12, 0x7c, 0x8c, 0x3f, 0xbc, 0x3c, 0x1b, 0xc5,
    0x69, 0x22, 0x97, 0xaa, 0x73, 0x0a, 0x0c, 0x8a, 0x90, 0x31, 0xc4, 0x33, 0xe1, 0x8b, 0x9c, 0x63,
    0x5f, 0xf5, 0xf7, 0xff, 0x79, 0x49, 0xd3, 0xc6, 0x7b, 0x1a, 0x39, 0xc8, 0x6e, 0x72, 0xd9, 0xc3,
    0x62, 0x28, 0xbd, 0xbb, 0xfa, 0x2e, 0xbf, 0x43, 0x06, 0x0b, 0x7a, 0x64, 0x5c, 0x92, 0x37, 0x3d,
    0x66, 0x26, 0x51, 0xef, 0x0f, 0xa9, 0x14, 0x70, 0x16, 0x17, 0x10, 0x19, 0x93, 0x09, 0x59, 0x15,
    0xfe, 0x4a, 0xcb, 0x2c, 0xcd, 0xb8, 0x94, 0xab, 0xdf, 0xa7, 0x0e, 0x30, 0xaf, 0x56, 0x23, 0xb1,
    0xb0, 0x58, 0x7d, 0xc2, 0x1d, 0x50, 0x20, 0x61, 0x25, 0x89, 0xa0, 0x6c, 0x11, 0x54, 0x98, 0xb7,
    0x18, 0x21, 0xad, 0x3e, 0xd2, 0xea, 0x40, 0xd6, 0xf4, 0xa4, 0x8f, 0xa8, 0x08, 0x57, 0xba, 0xee,
    0x75, 0x6a, 0x07, 0x99, 0x7f, 0x1c, 0xe3, 0x46, 0x67, 0xec, 0x27, 0x36, 0xb4, 0x65, 0x9e, 0x9a,
];

pub const ISB: [u8; 256] = [
    0x4d, 0x19, 0x36, 0x06, 0x23, 0x60, 0xa8, 0xf2, 0xec, 0xbd, 0x85, 0xa9, 0x86, 0x52, 0xca, 0xb4,
    0xba, 0xdc, 0x78, 0x68, 0xb6, 0xbf, 0xb8, 0xb9, 0xe0, 0xbb, 0x99, 0x7e, 0xf5, 0xd4, 0x42, 0x28,
    0xd6, 0xe1, 0x81, 0xce, 0x4e, 0xd8, 0xb1, 0xfa, 0xa1, 0x71, 0x40, 0x10, 0xc3, 0x66, 0xa5, 0x04,
    0xcb, 0x89, 0x6b, 0x8b, 0x67, 0x2d, 0xfb, 0xae, 0x6c, 0x9a, 0x27, 0x46, 0x7d, 0xaf, 0xe3, 0x7b,
    0xe6, 0x1e, 0x70, 0xa7, 0x3f, 0x1a, 0xf7, 0x64, 0x6a, 0x95, 0xc1, 0x4f, 0x14, 0x37, 0x21, 0x17,
    0xd5, 0xb2, 0x2f, 0x44, 0xdd, 0x5e, 0xcd, 0xed, 0xd1, 0xbe, 0x59, 0x63, 0xac, 0x41, 0x22, 0x90,
    0x0a, 0xd7, 0xa0, 0x8f, 0xab, 0xfd, 0xb0, 0xf8, 0x35, 0x80, 0xf1, 0x3d, 0xdb, 0x75, 0x9c, 0x15,
    0xb7, 0x45, 0x9d, 0x84, 0x03, 0xf0, 0x1b, 0x07, 0x30, 0x94, 0xaa, 0x98, 0x79, 0xd2, 0x54, 0xf4,
    0x4a, 0x3e, 0x18, 0x51, 0x4c, 0x2c, 0x55, 0x53, 0x69, 0xd9, 0x87, 0x8d, 0x7a, 0x77, 0x2a, 0xea,
    0x88, 0x13, 0xad, 0xbc, 0xc6, 0x0b, 0x57, 0x82, 0xde, 0xf3, 0xff, 0x74, 0x8e, 0x49, 0xfe, 0x1c,
    0xda, 0x31, 0x33, 0x3b, 0xe9, 0x12, 0x02, 0xc9, 0xeb, 0xb5, 0x83, 0xc7, 0x20, 0xe2, 0x08, 0xcc,
    0xd0, 0xcf, 0x05, 0x09, 0xfc, 0x01, 0x50, 0xdf, 0xc5, 0x6d, 0xee, 0xa3, 0x7c, 0xa2, 0x58, 0xa6,
    0x5f, 0x47, 0xd3, 0x9f, 0x8a, 0x7f, 0x97, 0x0e, 0x9b, 0x6f, 0x56, 0xc2, 0x48, 0xc4, 0x11, 0x3a,
    0x5b, 0x00, 0xe4, 0x96, 0x5c, 0x4b, 0xe7, 0x72, 0x5d, 0x9e, 0x6e, 0x2e, 0x3c, 0x39, 0x34, 0xc8,
    0x43, 0x8c, 0x29, 0xf6, 0x65, 0x61, 0x5a, 0x2b, 0x76, 0x62, 0xe5, 0x24, 0xf9, 0x1d, 0xef, 0xb3,
    0x0f, 0x26, 0x73, 0x16, 0xe8, 0x91, 0x38, 0x92, 0x0d, 0x25, 0xa4, 0x1f, 0x32, 0x0c, 0xc0, 0x93,
];

const C0: [u32; 3] = [1, 17, 14];
const C1: [u32; 7] = [3, 5, 11, 21, 16, 30, 19];
const C2: [u32; 7] = [4, 0, 22, 27, 47, 4, 61];

macro_rules! RNDS {
    ($x:expr) => {
        16 + ($x - 32) / 16
    };
}

use num_traits::Inv;
pub(crate) use RNDS;

#[allow(non_snake_case)]
#[inline]
pub fn ROTL(x: u32, s: u32) -> u32 {
    (x << s) | (x >> (32 - s))
}

#[allow(non_snake_case)]
#[inline]
pub fn ROTL64(x: u64, s: u32) -> u64 {
    x.wrapping_shl(s) | x.wrapping_shr(64 - s)
}

fn lin334(din: &[u32], dout: &mut [u32]) {
    let dout = dout.as_mut_ptr();
    let din = din.as_ptr();
    let c0 = C0.as_ptr();
    unsafe {
        *dout.add(0) = *din.add(0)
            ^ ROTL(*din.add(1), *c0.add(0))
            ^ ROTL(*din.add(2), *c0.add(1))
            ^ ROTL(*din.add(3), *c0.add(2));
        *dout.add(1) = *din.add(1)
            ^ ROTL(*din.add(2), *c0.add(0))
            ^ ROTL(*din.add(3), *c0.add(1))
            ^ ROTL(*dout.add(0), *c0.add(2));
        *dout.add(2) = *din.add(2)
            ^ ROTL(*din.add(3), *c0.add(0))
            ^ ROTL(*dout.add(0), *c0.add(1))
            ^ ROTL(*dout.add(1), *c0.add(2));
        *dout.add(3) = *din.add(3)
            ^ ROTL(*dout.add(0), *c0.add(0))
            ^ ROTL(*dout.add(1), *c0.add(1))
            ^ ROTL(*dout.add(2), *c0.add(2));
    }
}

fn lin384(din: &[u32], dout: &mut [u32]) {
    let dout = dout.as_mut_ptr();
    let din = din.as_ptr();
    let c1 = C1.as_ptr();
    unsafe {
        *dout.add(0) = *din.add(0)
            ^ ROTL(*din.add(1), *c1.add(0))
            ^ ROTL(*din.add(2), *c1.add(1))
            ^ ROTL(*din.add(3), *c1.add(2))
            ^ ROTL(*din.add(4), *c1.add(3))
            ^ ROTL(*din.add(5), *c1.add(4))
            ^ ROTL(*din.add(6), *c1.add(5))
            ^ ROTL(*din.add(7), *c1.add(6));
        *dout.add(1) = *din.add(1)
            ^ ROTL(*din.add(2), *c1.add(0))
            ^ ROTL(*din.add(3), *c1.add(1))
            ^ ROTL(*din.add(4), *c1.add(2))
            ^ ROTL(*din.add(5), *c1.add(3))
            ^ ROTL(*din.add(6), *c1.add(4))
            ^ ROTL(*din.add(7), *c1.add(5))
            ^ ROTL(*dout.add(0), *c1.add(6));
        *dout.add(2) = *din.add(2)
            ^ ROTL(*din.add(3), *c1.add(0))
            ^ ROTL(*din.add(4), *c1.add(1))
            ^ ROTL(*din.add(5), *c1.add(2))
            ^ ROTL(*din.add(6), *c1.add(3))
            ^ ROTL(*din.add(7), *c1.add(4))
            ^ ROTL(*dout.add(0), *c1.add(5))
            ^ ROTL(*dout.add(1), *c1.add(6));
        *dout.add(3) = *din.add(3)
            ^ ROTL(*din.add(4), *c1.add(0))
            ^ ROTL(*din.add(5), *c1.add(1))
            ^ ROTL(*din.add(6), *c1.add(2))
            ^ ROTL(*din.add(7), *c1.add(3))
            ^ ROTL(*dout.add(0), *c1.add(4))
            ^ ROTL(*dout.add(1), *c1.add(5))
            ^ ROTL(*dout.add(2), *c1.add(6));
        *dout.add(4) = *din.add(4)
            ^ ROTL(*din.add(5), *c1.add(0))
            ^ ROTL(*din.add(6), *c1.add(1))
            ^ ROTL(*din.add(7), *c1.add(2))
            ^ ROTL(*dout.add(0), *c1.add(3))
            ^ ROTL(*dout.add(1), *c1.add(4))
            ^ ROTL(*dout.add(2), *c1.add(5))
            ^ ROTL(*dout.add(3), *c1.add(6));
        *dout.add(5) = *din.add(5)
            ^ ROTL(*din.add(6), *c1.add(0))
            ^ ROTL(*din.add(7), *c1.add(1))
            ^ ROTL(*dout.add(0), *c1.add(2))
            ^ ROTL(*dout.add(1), *c1.add(3))
            ^ ROTL(*dout.add(2), *c1.add(4))
            ^ ROTL(*dout.add(3), *c1.add(5))
            ^ ROTL(*dout.add(4), *c1.add(6));
        *dout.add(6) = *din.add(6)
            ^ ROTL(*din.add(7), *c1.add(0))
            ^ ROTL(*dout.add(0), *c1.add(1))
            ^ ROTL(*dout.add(1), *c1.add(2))
            ^ ROTL(*dout.add(2), *c1.add(3))
            ^ ROTL(*dout.add(3), *c1.add(4))
            ^ ROTL(*dout.add(4), *c1.add(5))
            ^ ROTL(*dout.add(5), *c1.add(6));
        *dout.add(7) = *din.add(7)
            ^ ROTL(*dout.add(0), *c1.add(0))
            ^ ROTL(*dout.add(1), *c1.add(1))
            ^ ROTL(*dout.add(2), *c1.add(2))
            ^ ROTL(*dout.add(3), *c1.add(3))
            ^ ROTL(*dout.add(4), *c1.add(4))
            ^ ROTL(*dout.add(5), *c1.add(5))
            ^ ROTL(*dout.add(6), *c1.add(6));
    }
}

fn lin388(din: &[u64], dout: &mut [u64]) {
    let dout = dout.as_mut_ptr();
    let din = din.as_ptr();
    let c2 = C2.as_ptr();
    unsafe {
        *dout.add(0) = *din.add(0)
            ^ ROTL64(*din.add(1), *c2.add(0))
            ^ *din.add(2)
            ^ ROTL64(*din.add(3), *c2.add(2))
            ^ ROTL64(*din.add(4), *c2.add(3))
            ^ ROTL64(*din.add(5), *c2.add(4))
            ^ ROTL64(*din.add(6), *c2.add(5))
            ^ ROTL64(*din.add(7), *c2.add(6));

        *dout.add(1) = *din.add(1)
            ^ ROTL64(*din.add(2), *c2.add(0))
            ^ *din.add(3)
            ^ ROTL64(*din.add(4), *c2.add(2))
            ^ ROTL64(*din.add(5), *c2.add(3))
            ^ ROTL64(*din.add(6), *c2.add(4))
            ^ ROTL64(*din.add(7), *c2.add(5))
            ^ ROTL64(*dout.add(0), *c2.add(6));

        *dout.add(2) = *din.add(2)
            ^ ROTL64(*din.add(3), *c2.add(0))
            ^ *din.add(4)
            ^ ROTL64(*din.add(5), *c2.add(2))
            ^ ROTL64(*din.add(6), *c2.add(3))
            ^ ROTL64(*din.add(7), *c2.add(4))
            ^ ROTL64(*dout.add(0), *c2.add(5))
            ^ ROTL64(*dout.add(1), *c2.add(6));

        *dout.add(3) = *din.add(3)
            ^ ROTL64(*din.add(4), *c2.add(0))
            ^ *din.add(5)
            ^ ROTL64(*din.add(6), *c2.add(2))
            ^ ROTL64(*din.add(7), *c2.add(3))
            ^ ROTL64(*dout.add(0), *c2.add(4))
            ^ ROTL64(*dout.add(1), *c2.add(5))
            ^ ROTL64(*dout.add(2), *c2.add(6));

        *dout.add(4) = *din.add(4)
            ^ ROTL64(*din.add(5), *c2.add(0))
            ^ *din.add(6)
            ^ ROTL64(*din.add(7), *c2.add(2))
            ^ ROTL64(*dout.add(0), *c2.add(3))
            ^ ROTL64(*dout.add(1), *c2.add(4))
            ^ ROTL64(*dout.add(2), *c2.add(5))
            ^ ROTL64(*dout.add(3), *c2.add(6));

        *dout.add(5) = *din.add(5)
            ^ ROTL64(*din.add(6), *c2.add(0))
            ^ *din.add(7)
            ^ ROTL64(*dout.add(0), *c2.add(2))
            ^ ROTL64(*dout.add(1), *c2.add(3))
            ^ ROTL64(*dout.add(2), *c2.add(4))
            ^ ROTL64(*dout.add(3), *c2.add(5))
            ^ ROTL64(*dout.add(4), *c2.add(6));

        *dout.add(6) = *din.add(6)
            ^ ROTL64(*din.add(7), *c2.add(0))
            ^ *dout.add(0)
            ^ ROTL64(*dout.add(1), *c2.add(2))
            ^ ROTL64(*dout.add(2), *c2.add(3))
            ^ ROTL64(*dout.add(3), *c2.add(4))
            ^ ROTL64(*dout.add(4), *c2.add(5))
            ^ ROTL64(*dout.add(5), *c2.add(6));

        *dout.add(7) = *din.add(7)
            ^ ROTL64(*dout.add(0), *c2.add(0))
            ^ *dout.add(1)
            ^ ROTL64(*dout.add(2), *c2.add(2))
            ^ ROTL64(*dout.add(3), *c2.add(3))
            ^ ROTL64(*dout.add(4), *c2.add(4))
            ^ ROTL64(*dout.add(5), *c2.add(5))
            ^ ROTL64(*dout.add(6), *c2.add(6));
    }
}

pub fn linOp(d: &[u8], r: &mut [u8], blocklen: usize) {
    match blocklen {
        16 => {
            let dp = d.as_ptr() as *const u32;
            let din = unsafe { std::slice::from_raw_parts(dp, d.len() / 4) };
            let rp = r.as_mut_ptr() as *mut u32;
            let dout = unsafe { std::slice::from_raw_parts_mut(rp, r.len() / 4) };
            lin334(din, dout);
        }
        32 => {
            let dp = d.as_ptr() as *const u32;
            let din = unsafe { std::slice::from_raw_parts(dp, d.len() / 4) };
            let rp = r.as_mut_ptr() as *mut u32;
            let dout = unsafe { std::slice::from_raw_parts_mut(rp, r.len() / 4) };
            lin384(din, dout);
        }
        64 => {
            let dp = d.as_ptr() as *const u64;
            let din = unsafe { std::slice::from_raw_parts(dp, d.len() / 8) };
            let rp = r.as_mut_ptr() as *mut u64;
            let dout = unsafe { std::slice::from_raw_parts_mut(rp, r.len() / 8) };
            lin388(din, dout);
        }
        _ => panic!("Unsupported block length {}", blocklen),
    }
}

#[allow(non_snake_case)]
pub fn Kexp(key: &[u8], klen: usize, blen: usize, rkey: &mut [u8]) {
    let mut r0arr: [u8; 17] = [0; 17];
    let mut r1arr: [u8; 15] = [0; 15];
    let key = key.as_ptr();
    let r0 = r0arr.as_mut_ptr();
    let r1 = r1arr.as_mut_ptr();
    let addk = klen - 32;
    let mut step = 0;
    let mut s = SHIFT as usize;
    let rkey = rkey.as_mut_ptr();
    unsafe {
        for i in 0..15usize {
            *r0.add(i) = *key.add(2 * i);
            *r1.add(i) = *key.add(2 * i + 1);
        }
        *r0.add(15) = *key.add(30);
        *r0.add(16) = *key.add(31);
        let sb = SB.as_ptr();
        for r in 0..RNDS!(klen) {
            for k in 0..(blen + s) {
                let mut t0: u8 = (*sb.add(*r0.add(0) as usize))
                    .wrapping_add(*r0.add(1))
                    .wrapping_add(*sb.add(*r0.add(3) as usize))
                    .wrapping_add(*r0.add(7))
                    .wrapping_add(*sb.add(*r0.add(12) as usize))
                    .wrapping_add(*r0.add(16));

                let mut t1: u8 = (*sb.add(*r1.add(0) as usize))
                    .wrapping_add(*r1.add(3))
                    .wrapping_add(*sb.add(*r1.add(9) as usize))
                    .wrapping_add(*r1.add(12))
                    .wrapping_add(*sb.add(*r1.add(14) as usize));
                for i in 0..14usize {
                    *r0.add(i) = *r0.add(i + 1);
                    *r1.add(i) = *r1.add(i + 1);
                }
                *r0.add(14) = *r0.add(15);
                *r0.add(15) = *r0.add(16);
                if k >= s {
                    let rkey_idx = r * blen + k - s;
                    *rkey.add(rkey_idx) = t0.wrapping_add(*r1.add(4));
                    if step < addk {
                        if step & 1 != 0 {
                            t0 = t0.wrapping_add(*key.add(32 + step));
                        } else {
                            t1 = t1.wrapping_add(*key.add(32 + step));
                        }
                        step += 1;
                    }
                }
                *r0.add(16) = t0;
                *r1.add(14) = t1;
            }
            s = 0;
        }
    }
}

pub fn sBox(data: &[u8], res: &mut [u8], blen: usize) {
    data.iter()
        .take(blen)
        .zip(res.iter_mut().take(blen))
        .for_each(|(d, r)| {
            *r = SB[*d as usize];
        });
}

pub fn sBoxU(data: &[u8], res: &mut [u8], blen: usize) {
    let res = res.as_mut_ptr();
    let sb = SB.as_ptr();
    let data = data.as_ptr();
    unsafe {
        for i in 0..blen {
            *res.add(i) = *sb.add(*data.add(i) as usize);
        }
    }
}

pub fn AddRkX(block: &[u8], rkey: &[u8], nr: usize, blen: usize, res: &mut [u8]) {
    block
        .iter()
        .zip(rkey.iter().skip(nr * blen).take(blen))
        .zip(res.iter_mut())
        .for_each(|((b, k), r)| {
            *r = *b ^ *k;
        });
}

pub fn AddRk(block: &[u8], rkey: &[u8], nr: usize, blen: usize, res: &mut [u8]) {
    let block = block.as_ptr();
    let rkey = rkey.as_ptr();
    let res = res.as_mut_ptr();
    unsafe {
        let mut tmp: u16 = *block.add(0) as u16 + *rkey.add(nr * blen) as u16;
        *res.add(0) = tmp as u8;
        tmp >>= 8;
        for i in 1..blen {
            tmp += *block.add(i) as u16 + *rkey.add(blen * nr + i) as u16;
            *res.add(i) = tmp as u8;
            tmp >>= 8;
        }
    }
}

fn encrypt(data: &[u8], rkey: &[u8], klen: usize, blen: usize, res: &mut [u8]) {
    let mut block: [u8; MAXBLOCKLEN as usize] = [0; MAXBLOCKLEN as usize];
    let mut block2: [u8; MAXBLOCKLEN as usize] = [0; MAXBLOCKLEN as usize];
    AddRk(&data, &rkey, 0, blen as usize, &mut block);
    sBox(&block, &mut block2, blen as usize);
    linOp(&block2, &mut block, blen as usize);
    for i in 1..RNDS!(klen) - 1 {
        AddRkX(&block, &rkey, i as usize, blen as usize, &mut block2);
        let b2s: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(block2.as_mut_ptr(), blen as usize) };
        sBox(&block2, b2s, blen as usize);
        linOp(&block2, &mut block, blen as usize);
    }
    AddRk(&block, &rkey, RNDS!(klen) - 1, blen, res);
}

pub fn InvAddRk(block: &[u8], rkey: &[u8], nr: usize, blen: usize, res: &mut [u8]) {
    let block = block.as_ptr();
    let rkey = rkey.as_ptr();
    let res = res.as_mut_ptr();
    unsafe {
        let mut tmp: u16 = *block.add(0) as u16 - *rkey.add(nr * blen) as u16;
        *res.add(0) = tmp as u8;
        tmp >>= 8;
        for i in 1..blen {
            tmp += *block.add(i) as u16 - *rkey.add(blen * nr + i) as u16;
            *res.add(i) = tmp as u8;
            tmp >>= 8;
        }
    }
}

fn ilin334(din: &[u32], dout: &mut [u32]) {
    let dout = dout.as_mut_ptr();
    let din = din.as_ptr();
    let c0 = C0.as_ptr();
    unsafe {
        *dout.add(3) = *din.add(3)
            ^ ROTL(*din.add(0), *c0.add(0))
            ^ ROTL(*din.add(1), *c0.add(1))
            ^ ROTL(*din.add(2), *c0.add(2));
        *dout.add(2) = *din.add(2)
            ^ ROTL(*dout.add(3), *c0.add(2))
            ^ ROTL(*din.add(0), *c0.add(0))
            ^ ROTL(*din.add(1), *c0.add(1));
        *dout.add(1) = *din.add(1)
            ^ ROTL(*dout.add(2), *c0.add(1))
            ^ ROTL(*dout.add(3), *c0.add(2))
            ^ ROTL(*din.add(0), *c0.add(0));
        *dout.add(0) = *din.add(0)
            ^ ROTL(*dout.add(1), *c0.add(0))
            ^ ROTL(*dout.add(2), *c0.add(1))
            ^ ROTL(*dout.add(3), *c0.add(2));
    }
}

fn ilin384(din: &[u32], dout: &mut [u32]) {
    let dout = dout.as_mut_ptr();
    let din = din.as_ptr();
    let c1 = C1.as_ptr();
    unsafe {
        *dout.add(7) = *din.add(7)
            ^ ROTL(*din.add(0), *c1.add(0))
            ^ ROTL(*din.add(1), *c1.add(1))
            ^ ROTL(*din.add(2), *c1.add(2))
            ^ ROTL(*din.add(3), *c1.add(3))
            ^ ROTL(*din.add(4), *c1.add(4))
            ^ ROTL(*din.add(5), *c1.add(5))
            ^ ROTL(*din.add(6), *c1.add(6));

        *dout.add(6) = *din.add(6)
            ^ ROTL(*dout.add(7), *c1.add(6))
            ^ ROTL(*din.add(0), *c1.add(0))
            ^ ROTL(*din.add(1), *c1.add(1))
            ^ ROTL(*din.add(2), *c1.add(2))
            ^ ROTL(*din.add(3), *c1.add(3))
            ^ ROTL(*din.add(4), *c1.add(4))
            ^ ROTL(*din.add(5), *c1.add(5));

        *dout.add(5) = *din.add(5)
            ^ ROTL(*dout.add(6), *c1.add(5))
            ^ ROTL(*dout.add(7), *c1.add(6))
            ^ ROTL(*din.add(0), *c1.add(0))
            ^ ROTL(*din.add(1), *c1.add(1))
            ^ ROTL(*din.add(2), *c1.add(2))
            ^ ROTL(*din.add(3), *c1.add(3))
            ^ ROTL(*din.add(4), *c1.add(4));

        *dout.add(4) = *din.add(4)
            ^ ROTL(*dout.add(5), *c1.add(4))
            ^ ROTL(*dout.add(6), *c1.add(5))
            ^ ROTL(*dout.add(7), *c1.add(6))
            ^ ROTL(*din.add(0), *c1.add(0))
            ^ ROTL(*din.add(1), *c1.add(1))
            ^ ROTL(*din.add(2), *c1.add(2))
            ^ ROTL(*din.add(3), *c1.add(3));

        *dout.add(3) = *din.add(3)
            ^ ROTL(*dout.add(4), *c1.add(3))
            ^ ROTL(*dout.add(5), *c1.add(4))
            ^ ROTL(*dout.add(6), *c1.add(5))
            ^ ROTL(*dout.add(7), *c1.add(6))
            ^ ROTL(*din.add(0), *c1.add(0))
            ^ ROTL(*din.add(1), *c1.add(1))
            ^ ROTL(*din.add(2), *c1.add(2));

        *dout.add(2) = *din.add(2)
            ^ ROTL(*dout.add(3), *c1.add(2))
            ^ ROTL(*dout.add(4), *c1.add(3))
            ^ ROTL(*dout.add(5), *c1.add(4))
            ^ ROTL(*dout.add(6), *c1.add(5))
            ^ ROTL(*dout.add(7), *c1.add(6))
            ^ ROTL(*din.add(0), *c1.add(0))
            ^ ROTL(*din.add(1), *c1.add(1));

        *dout.add(1) = *din.add(1)
            ^ ROTL(*dout.add(2), *c1.add(1))
            ^ ROTL(*dout.add(3), *c1.add(2))
            ^ ROTL(*dout.add(4), *c1.add(3))
            ^ ROTL(*dout.add(5), *c1.add(4))
            ^ ROTL(*dout.add(6), *c1.add(5))
            ^ ROTL(*dout.add(7), *c1.add(6))
            ^ ROTL(*din.add(0), *c1.add(0));

        *dout.add(0) = *din.add(0)
            ^ ROTL(*dout.add(1), *c1.add(0))
            ^ ROTL(*dout.add(2), *c1.add(1))
            ^ ROTL(*dout.add(3), *c1.add(2))
            ^ ROTL(*dout.add(4), *c1.add(3))
            ^ ROTL(*dout.add(5), *c1.add(4))
            ^ ROTL(*dout.add(6), *c1.add(5))
            ^ ROTL(*dout.add(7), *c1.add(6));
    }
}

fn ilin388(din: &[u64], dout: &mut [u64]) {
    let dout = dout.as_mut_ptr();
    let din = din.as_ptr();
    let c2 = C2.as_ptr();
    unsafe {
        *dout.add(7) = *din.add(7)
            ^ ROTL64(*din.add(0), *c2.add(0))
            ^ *din.add(1)
            ^ ROTL64(*din.add(2), *c2.add(2))
            ^ ROTL64(*din.add(3), *c2.add(3))
            ^ ROTL64(*din.add(4), *c2.add(4))
            ^ ROTL64(*din.add(5), *c2.add(5))
            ^ ROTL64(*din.add(6), *c2.add(6));

        *dout.add(6) = *din.add(6)
            ^ ROTL64(*dout.add(7), *c2.add(0))
            ^ ROTL64(*din.add(0), *c2.add(1))
            ^ ROTL64(*din.add(1), *c2.add(2))
            ^ ROTL64(*din.add(2), *c2.add(3))
            ^ ROTL64(*din.add(3), *c2.add(4))
            ^ ROTL64(*din.add(4), *c2.add(5))
            ^ ROTL64(*din.add(5), *c2.add(6));

        *dout.add(5) = *din.add(5)
            ^ ROTL64(*dout.add(6), *c2.add(0))
            ^ ROTL64(*dout.add(7), *c2.add(1))
            ^ ROTL64(*din.add(0), *c2.add(2))
            ^ ROTL64(*din.add(1), *c2.add(3))
            ^ ROTL64(*din.add(2), *c2.add(4))
            ^ ROTL64(*din.add(3), *c2.add(5))
            ^ ROTL64(*din.add(4), *c2.add(6));

        *dout.add(4) = *din.add(4)
            ^ ROTL64(*dout.add(5), *c2.add(0))
            ^ ROTL64(*dout.add(6), *c2.add(1))
            ^ ROTL64(*dout.add(7), *c2.add(2))
            ^ ROTL64(*din.add(0), *c2.add(3))
            ^ ROTL64(*din.add(1), *c2.add(4))
            ^ ROTL64(*din.add(2), *c2.add(5))
            ^ ROTL64(*din.add(3), *c2.add(6));

        *dout.add(3) = *din.add(3)
            ^ ROTL64(*dout.add(4), *c2.add(0))
            ^ ROTL64(*dout.add(5), *c2.add(1))
            ^ ROTL64(*dout.add(6), *c2.add(2))
            ^ ROTL64(*dout.add(7), *c2.add(3))
            ^ ROTL64(*din.add(0), *c2.add(4))
            ^ ROTL64(*din.add(1), *c2.add(5))
            ^ ROTL64(*din.add(2), *c2.add(6));

        *dout.add(2) = *din.add(2)
            ^ ROTL64(*dout.add(3), *c2.add(0))
            ^ ROTL64(*dout.add(4), *c2.add(1))
            ^ ROTL64(*dout.add(5), *c2.add(2))
            ^ ROTL64(*dout.add(6), *c2.add(3))
            ^ ROTL64(*dout.add(7), *c2.add(4))
            ^ ROTL64(*din.add(0), *c2.add(5))
            ^ ROTL64(*din.add(1), *c2.add(6));

        *dout.add(1) = *din.add(1)
            ^ ROTL64(*dout.add(2), *c2.add(0))
            ^ ROTL64(*dout.add(3), *c2.add(1))
            ^ ROTL64(*dout.add(4), *c2.add(2))
            ^ ROTL64(*dout.add(5), *c2.add(3))
            ^ ROTL64(*dout.add(6), *c2.add(4))
            ^ ROTL64(*dout.add(7), *c2.add(5))
            ^ ROTL64(*din.add(0), *c2.add(6));

        *dout.add(0) = *din.add(0)
            ^ ROTL64(*dout.add(1), *c2.add(0))
            ^ ROTL64(*dout.add(2), *c2.add(1))
            ^ ROTL64(*dout.add(3), *c2.add(2))
            ^ ROTL64(*dout.add(4), *c2.add(3))
            ^ ROTL64(*dout.add(5), *c2.add(4))
            ^ ROTL64(*dout.add(6), *c2.add(5))
            ^ ROTL64(*dout.add(7), *c2.add(6));
    }
}

pub fn InvlinOp(d: &[u8], r: &mut [u8], blocklen: usize) {
    match blocklen {
        16 => {
            let dp = d.as_ptr() as *const u32;
            let din = unsafe { std::slice::from_raw_parts(dp, d.len() / 4) };
            let rp = r.as_mut_ptr() as *mut u32;
            let dout = unsafe { std::slice::from_raw_parts_mut(rp, r.len() / 4) };
            ilin334(din, dout);
        }
        32 => {
            let dp = d.as_ptr() as *const u32;
            let din = unsafe { std::slice::from_raw_parts(dp, d.len() / 4) };
            let rp = r.as_mut_ptr() as *mut u32;
            let dout = unsafe { std::slice::from_raw_parts_mut(rp, r.len() / 4) };
            ilin384(din, dout);
        }
        64 => {
            let dp = d.as_ptr() as *const u64;
            let din = unsafe { std::slice::from_raw_parts(dp, d.len() / 8) };
            let rp = r.as_mut_ptr() as *mut u64;
            let dout = unsafe { std::slice::from_raw_parts_mut(rp, r.len() / 8) };
            ilin388(din, dout);
        }
        _ => panic!("Unsupported block length {}", blocklen),
    }
}

pub fn InvsBox(data: &[u8], res: &mut [u8], blen: usize) {
    data.iter()
        .take(blen)
        .zip(res.iter_mut().take(blen))
        .for_each(|(d, r)| {
            *r = ISB[*d as usize];
        });
}

fn decrypt(data: &[u8], rkey: &[u8], klen: usize, blen: usize, res: &mut [u8]) {
    let mut block: [u8; MAXBLOCKLEN as usize] = [0; MAXBLOCKLEN as usize];
    let mut block2: [u8; MAXBLOCKLEN as usize] = [0; MAXBLOCKLEN as usize];
    InvAddRk(&data, &rkey, RNDS!(klen) - 1, blen as usize, &mut block);
    for i in (1..RNDS!(klen) - 1).rev() {
        InvlinOp(&block, &mut block2, blen as usize);
        let b2s: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(block2.as_mut_ptr(), blen as usize) };
        InvsBox(&block2, b2s, blen as usize);
        AddRk(&block2, &rkey, i as usize, blen as usize, &mut block);
    }
    InvlinOp(&block, &mut block2, blen as usize);
    let b2s: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(block2.as_mut_ptr(), blen as usize) };
    InvsBox(&block2, b2s, blen as usize);
    InvAddRk(&block2, rkey, 0, blen, res);
}
