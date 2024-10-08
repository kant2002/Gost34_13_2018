use sprintf::sprintf;
use std::io::{Read, Write};

const BNTBLEN_CNT: usize = 3;

macro_rules! fprint {
    ($x:expr, $y:expr) => {
        match $x {
            Some(ref mut _fr) => {
                if let Err(e) = _fr.write_all($y) {
                    eprintln!("Error: {} [{}:{}]", e, file!(), line!());
                }
            }
            None => {}
        }
    };
}

fn short_test_lin<W: Write, R: Read>(f: &mut Option<&mut W>, src: &mut Option<&mut R>) {
    let mut din: [u8; super::MAXBLOCKLEN as usize] = [0; super::MAXBLOCKLEN as usize];
    let mut dout: [u8; super::MAXBLOCKLEN as usize] = [0; super::MAXBLOCKLEN as usize];
    if src.is_none() {
        din.iter_mut().for_each(|x| *x = super::rnext());
    } else {
        match src {
            Some(ref mut _fr) => {
                if let Err(e) = _fr.read_exact(&mut din) {
                    eprintln!("Error: {} [{}:{}]", e, file!(), line!());
                    return;
                }
            }
            None => {}
        }
    }

    let blen_vals: [i32; BNTBLEN_CNT] = [16, 32, 64];
    fprint!(f, b"Linear operation:\n");
    for ex in 0..blen_vals.len() {
        fprint!(
            f,
            sprintf!("Input (%d bits):  ", blen_vals[ex] * 8)
                .unwrap()
                .as_bytes()
        );
        for i in 0..blen_vals[ex] as usize {
            fprint!(f, sprintf!("%02X", din[i]).unwrap().as_bytes());
        }
        fprint!(
            f,
            sprintf!("\nOutput (%d bits): ", blen_vals[ex] * 8)
                .unwrap()
                .as_bytes()
        );
        super::linOp(&din[..], &mut dout[..], blen_vals[ex]);
        for i in 0..blen_vals[ex] as usize {
            fprint!(f, sprintf!("%02X", dout[i]).unwrap().as_bytes());
        }
        fprint!(f, b"\n");
    }
    fprint!(f, b"\n");
}

pub fn short_test_vectors<W: Write, R: Read>(mut f: Option<&mut W>, mut src: Option<&mut R>) {
    short_test_lin(&mut f, &mut src);
    short_test_lin(&mut f, &mut src);
}
