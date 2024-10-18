use super::qalqan::{KEYLENSTEP, MAXBLOCKLEN, MAXKEYLEN, MINBLOCKLEN, MINKEYLEN, RNDS, SB, SHIFT};
use sprintf::sprintf;
use std::io::{Read, Write};

const BNTBLEN_CNT: usize = 3;

macro_rules! fprintf {
    ($x:expr, $y:expr) => {
        match $x {
            Some(ref mut _fr) => {
                if let Err(e) = _fr.write_all($y.as_bytes()) {
                    eprintln!("Error: {} [{}:{}]", e, file!(), line!());
                }
            }
            None => {}
        }
    };
    ($x:expr, $y:expr, $($z:expr),*) => {
        match $x {
            Some(ref mut _fr) => {
                let __s = sprintf!($y, $($z),*).unwrap();
                if let Err(e) = _fr.write_all(__s.as_bytes()) {
                    eprintln!("Error: {} [{}:{}]", e, file!(), line!());
                }
            }
            None => {}
        }
    };
}

#[allow(non_snake_case)]
fn Pr3<W: Write>(b: &[u8], f: &mut Option<&mut W>) {
    b.iter().for_each(|x| fprintf!(f, "%02x", *x));
    fprintf!(f, "\n");
}

#[allow(non_snake_case)]
fn Pr<W: Write>(msg: &str, b: &[u8], f: &mut Option<&mut W>) {
    fprintf!(f, "%s:\n", msg);
    b.iter().for_each(|x| fprintf!(f, "%02x", *x));
    fprintf!(f, "\n");
}

#[allow(non_snake_case)]
fn KexpV<W: Write>(key: &mut [u8], klen: i32, blen: i32, rkey: &mut [u8], f: &mut Option<&mut W>) {
    Pr("Initial key", key, f);
    let mut r0: [u8; 17] = [0; 17];
    let mut r1: [u8; 15] = [0; 15];
    let addk = klen - 32;
    let mut step = 0;
    let mut s = SHIFT;
    r0.iter_mut()
        .take(15)
        .enumerate()
        .for_each(|x| *x.1 = key[2 * x.0]);
    r1.iter_mut()
        .enumerate()
        .for_each(|x| *x.1 = key[2 * x.0 + 1]);
    r0[15] = key[30];
    r0[16] = key[31];
    Pr("Register L0\0", &r0, f);
    Pr("Register L1\0", &r1, f);
    for r in 0..RNDS!(klen) {
        for k in 0..(blen + s) {
            let mut t0 = SB[r0[0] as usize]
                .wrapping_add(r0[1])
                .wrapping_add(SB[r0[3] as usize])
                .wrapping_add(r0[7])
                .wrapping_add(SB[r0[12] as usize])
                .wrapping_add(r0[16]);
            let mut t1 = SB[r1[0] as usize]
                .wrapping_add(r1[3])
                .wrapping_add(SB[r1[9] as usize])
                .wrapping_add(r1[12])
                .wrapping_add(SB[r1[14] as usize]);
            for i in 0..14 {
                r0[i] = r0[i + 1];
                r1[i] = r1[i + 1];
            }
            r0[14] = r0[15];
            r0[15] = r0[16];
            if k >= s {
                rkey[(r * blen + k - s) as usize] = t0.wrapping_add(r1[4]);
                if step < addk {
                    if step & 1 != 0 {
                        t0 = t0.wrapping_add(key[(32 + step) as usize]);
                    } else {
                        t1 = t1.wrapping_add(key[(32 + step) as usize]);
                    }
                    step += 1;
                }
            }
            r0[16] = t0;
            r1[14] = t1;
        }
        if s != 0 {
            fprintf!(f, "After prerun:\n");
            Pr("Register L0\0", &r0, f);
            Pr("Register L1\0", &r1, f);
        }
        s = 0;
    }
    for r in 0..RNDS!(klen) {
        fprintf!(f, "Round %2d key: ", r);
        Pr3(&rkey[(r * blen) as usize..(r * blen + blen) as usize], f);
    }
}

#[allow(non_snake_case)]
fn KexpVV<W: Write>(key: &mut [u8], klen: i32, blen: i32, rkey: &mut [u8], f: &mut Option<&mut W>) {
    Pr("Initial key", key, f);
    let mut r0: [u8; 17] = [0; 17];
    let mut r1: [u8; 15] = [0; 15];
    let addk = klen - 32;
    let mut step = 0;
    let mut s = SHIFT;
    let mut rkey_idxes: Vec<usize> = Vec::with_capacity(rkey.len());

    r0.iter_mut()
        .take(15)
        .enumerate()
        .for_each(|x| *x.1 = key[2 * x.0]);
    r1.iter_mut()
        .enumerate()
        .for_each(|x| *x.1 = key[2 * x.0 + 1]);
    r0[15] = key[30];
    r0[16] = key[31];
    Pr("Register L0\0", &r0, f);
    Pr("Register L1\0", &r1, f);
    for r in 0..RNDS!(klen) {
        let mut t0vec: Vec<u8> = Vec::with_capacity(32);
        let mut t1vec: Vec<u8> = Vec::with_capacity(32);
        for k in 0..(blen + s) {
            let mut t0 = SB[r0[0] as usize]
                .wrapping_add(r0[1])
                .wrapping_add(SB[r0[3] as usize])
                .wrapping_add(r0[7])
                .wrapping_add(SB[r0[12] as usize])
                .wrapping_add(r0[16]);

            let mut t1 = SB[r1[0] as usize]
                .wrapping_add(r1[3])
                .wrapping_add(SB[r1[9] as usize])
                .wrapping_add(r1[12])
                .wrapping_add(SB[r1[14] as usize]);

            if k < t0vec.capacity() as i32 {
                t0vec.push(t0);
                t1vec.push(t1);
            }
            for i in 0..14 {
                r0[i] = r0[i + 1];
                r1[i] = r1[i + 1];
            }
            r0[14] = r0[15];
            r0[15] = r0[16];
            if k >= s {
                let rkey_idx = (r * blen + k - s) as usize;
                rkey[rkey_idx] = t0.wrapping_add(r1[4]);
                rkey_idxes.push(rkey_idx);
                if step < addk {
                    if step & 1 != 0 {
                        if step < 32 {
                            fprintf!(
                                f,
                                "Additional key byte to L0: %02x\n",
                                key[(32 + step) as usize]
                            );
                        }
                        t0 = t0.wrapping_add(key[(32 + step) as usize]);
                    } else {
                        if step < 32 {
                            fprintf!(
                                f,
                                "Additional key byte to L1: %02x\n",
                                key[(32 + step) as usize]
                            );
                        }
                        t1 = t1.wrapping_add(key[(32 + step) as usize]);
                    }
                    if step < 32 {
                        Pr("Register L0\0", &r0, f);
                        Pr("Register L1\0", &r1, f);
                    }
                    step += 1;
                }
                if step < 8 {
                    fprintf!(
                        f,
                        "Output %02x (L0: %02x, L1: %02x)\n",
                        t0.wrapping_add(r1[4]),
                        t0,
                        r1[4]
                    );
                }
            }
            r0[16] = t0;
            r1[14] = t1;
            if step < 16 {
                fprintf!(f, "Step %d\nFeedback L0: %02x, L1: %02x\n", k, t0, t1);
                Pr("Register L0\0", &r0, f);
                Pr("Register L1\0", &r1, f);
            }
        }
        s = 0;
    }
    for r in 0..RNDS!(klen) {
        fprintf!(f, "Round %2d key: ", r);
        Pr3(&rkey[(r * blen) as usize..(r * blen + blen) as usize], f);
    }
    //println!("rkey Idx: {}", rkey_idxes.iter().map(|x| format!("{:03} ", x)).collect::<String>());
}

#[allow(non_snake_case)]
fn encryptV<W: Write>(
    data: &[u8],
    rkey: &[u8],
    klen: i32,
    blen: i32,
    res: &mut [u8],
    f: &mut Option<&mut W>,
) {
    let mut block: [u8; MAXBLOCKLEN as usize] = [0; MAXBLOCKLEN as usize];
    let mut block2: [u8; MAXBLOCKLEN as usize] = [0; MAXBLOCKLEN as usize];
    Pr("Clear text", data, f);
    Pr("Key 0", rkey, f);
    //super::cc_AddRk(&data, &rkey, 0, blen, &mut block);
    super::qalqan::AddRk(data, rkey, 0, blen as usize, &mut block);
    Pr("After add K0", &block, f);
    super::qalqan::sBox(&block, &mut block2, blen as usize);
    Pr("After Sub", &block2, f);
    //super::cc_linOp(&block2, &mut block, blen);
    super::qalqan::linOp(&block2, &mut block, blen as usize);
    Pr("After linear", &block, f);
    for i in 1..RNDS!(klen) - 1 {
        fprintf!(f, "Round %d.\nKey%d:\n", i, i);
        Pr3(&rkey[(i * blen) as usize..(i * blen + blen) as usize], f);
        //super::cc_AddRkX(&block, &rkey, i, blen, &mut block2);
        super::qalqan::AddRkX(&block, rkey, i as usize, blen as usize, &mut block2);
        Pr("After addkey", &block2, f);
        let b2 = block2.as_mut_ptr();
        let b2s: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(b2, blen as usize) };
        //super::cc_sBox(&mut block2, b2s, blen);
        super::qalqan::sBox(&block2, b2s, blen as usize);
        Pr("After Sub", &block2, f);
        //super::cc_linOp(&block2, &mut block, blen);
        super::qalqan::linOp(&block2, &mut block, blen as usize);
        Pr("After linear", &block, f);
    }
    Pr(
        "Final key",
        &rkey[((RNDS!(klen) - 1) * blen) as usize..(((RNDS!(klen) - 1) * blen) + blen) as usize],
        f,
    );
    super::qalqan::AddRk(&block, rkey, RNDS!(klen as usize) - 1, blen as usize, res);
    Pr("Ciphertext", res, f);
}

fn short_test_lin<W: Write, R: Read>(f: &mut Option<&mut W>, src: &mut Option<&mut R>) {
    let mut din: [u8; MAXBLOCKLEN as usize] = [0; MAXBLOCKLEN as usize];
    let mut dout: [u8; MAXBLOCKLEN as usize] = [0; MAXBLOCKLEN as usize];

    if let Some(ref mut _fr) = src {
        if let Err(e) = _fr.read_exact(&mut din) {
            eprintln!("Error: {} [{}:{}]", e, file!(), line!());
            return;
        }
    } else {
        din.iter_mut().for_each(|x| *x = super::prng::rnext());
    }

    let blen_vals: [usize; BNTBLEN_CNT] = [16, 32, 64];
    fprintf!(f, "Linear operation:\n");
    for ex in blen_vals.iter() {
        fprintf!(f, "Input (%d bits):  ", *ex * 8);
        for b in din.iter().take(*ex) {
            fprintf!(f, "%02x", *b);
        }
        fprintf!(f, "\nOutput (%d bits): ", *ex * 8);
        super::qalqan::linOp(&din[..], &mut dout[..], *ex);
        for b in dout.iter().take(*ex) {
            fprintf!(f, "%02x", *b);
        }
        fprintf!(f, "\n");
    }
    fprintf!(f, "\n");
}

//ShortTestSBox
fn short_test_sbox<W: Write, R: Read>(f: &mut Option<&mut W>, src: &mut Option<&mut R>) {
    let mut din: [u8; MAXBLOCKLEN as usize] = [0; MAXBLOCKLEN as usize];
    let mut dout: [u8; MAXBLOCKLEN as usize] = [0; MAXBLOCKLEN as usize];

    if let Some(ref mut _fr) = src {
        if let Err(e) = _fr.read_exact(&mut din) {
            eprintln!("Error: {} [{}:{}]", e, file!(), line!());
            return;
        }
    } else {
        din.iter_mut().for_each(|x| *x = super::prng::rnext());
    }

    dout.iter_mut()
        .enumerate()
        .for_each(|x| *x.1 = crate::qalqan::SB[din[x.0] as usize]);
    fprintf!(f, "Nonlinear operation:\n");
    fprintf!(f, "Input (%d bits):  ", MAXBLOCKLEN * 8);
    din.iter().for_each(|x| fprintf!(f, "%02x", *x));
    fprintf!(f, "\nOutput (%d bits): ", MAXBLOCKLEN * 8);
    dout.iter().for_each(|x| fprintf!(f, "%02x", *x));
    fprintf!(f, "\n\n");
}

//ShortTestKExp
fn short_test_kexp<W: Write, R: Read>(f: &mut Option<&mut W>, src: &mut Option<&mut R>) {
    let mut key: [u8; MAXKEYLEN as usize] = [0; MAXKEYLEN as usize];
    let mut rkey: [u8; (RNDS!(MAXKEYLEN) * MAXBLOCKLEN) as usize] =
        [0; (RNDS!(MAXKEYLEN) * MAXBLOCKLEN) as usize];
    if let Some(ref mut _fr) = src {
        if let Err(e) = _fr.read_exact(&mut key) {
            eprintln!("Error: {} [{}:{}]", e, file!(), line!());
            return;
        }
    } else {
        key.iter_mut().for_each(|x| *x = super::prng::rnext());
    }
    fprintf!(
        f,
        "\nKey expansion for %d bit key and %d bit block:\n",
        MAXKEYLEN * 8,
        MAXBLOCKLEN * 8
    );
    KexpVV(&mut key, MAXKEYLEN, MAXBLOCKLEN, &mut rkey, f);
    fprintf!(f, "\n");
}

//ShortTestEnc
fn short_test_enc<W: Write, R: Read>(f: &mut Option<&mut W>, src: &mut Option<&mut R>) {
    let mut key: [u8; MAXKEYLEN as usize] = [0; MAXKEYLEN as usize];
    let mut rkey: [u8; (RNDS!(MAXKEYLEN) * MAXBLOCKLEN) as usize] =
        [0; (RNDS!(MAXKEYLEN) * MAXBLOCKLEN) as usize];
    let mut data: [u8; MAXBLOCKLEN as usize] = [0; MAXBLOCKLEN as usize];
    let mut cipher: [u8; MAXBLOCKLEN as usize] = [0; MAXBLOCKLEN as usize];
    let blen_vals: [i32; BNTBLEN_CNT] = [16, 32, 64];

    if let Some(ref mut _fr) = src {
        if let Err(e) = _fr.read_exact(&mut data) {
            eprintln!("Error: {} [{}:{}]", e, file!(), line!());
            return;
        }
        if let Err(e) = _fr.read_exact(&mut key) {
            eprintln!("Error: {} [{}:{}]", e, file!(), line!());
            return;
        }
    } else {
        data.iter_mut().for_each(|x| *x = super::prng::rnext());
        key.iter_mut().for_each(|x| *x = super::prng::rnext());
    }

    for klen in (MINKEYLEN..MAXKEYLEN + KEYLENSTEP).step_by(KEYLENSTEP as usize) {
        fprintf!(
            f,
            "\n***** Key expansion, key length = %d, block length = %d *****\n",
            klen * 8,
            MINBLOCKLEN * 8
        );
        KexpV(&mut key, klen, MINBLOCKLEN, &mut rkey, f);
        fprintf!(f, "\n");
    }

    for blen in blen_vals.iter() {
        data.fill(0);
        cipher.fill(0);
        super::qalqan::Kexp(&key, MINKEYLEN as usize, (*blen) as usize, &mut rkey);
        fprintf!(
            f,
            "\n***** Encryption, key length = %d, block len = %d *****\n",
            MINKEYLEN * 8,
            blen * 8
        );
        encryptV(&data, &rkey, MINKEYLEN, *blen, &mut cipher, f);
        fprintf!(f, "\n");
    }
}

pub fn short_test_vectors<W: Write, R: Read>(mut f: Option<&mut W>, mut src: Option<&mut R>) {
    short_test_lin(&mut f, &mut src);
    short_test_lin(&mut f, &mut src);
    short_test_sbox(&mut f, &mut src);
    short_test_kexp(&mut f, &mut src);
    short_test_enc(&mut f, &mut src);
}
