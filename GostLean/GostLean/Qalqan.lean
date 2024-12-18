import GostLean.Support

def sb : Array UInt8 :=
    #[ -- ded: OK, dif: 4, dip: 7, lin: 32, pow: 7, cor: 0, dst: 112, sac: 116..140, cyc: 256
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
        0x75, 0x6a, 0x07, 0x99, 0x7f, 0x1c, 0xe3, 0x46, 0x67, 0xec, 0x27, 0x36, 0xb4, 0x65, 0x9e, 0x9a
    ]
def isb : Array UInt8 :=
    #[
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
        0x0f, 0x26, 0x73, 0x16, 0xe8, 0x91, 0x38, 0x92, 0x0d, 0x25, 0xa4, 0x1f, 0x32, 0x0c, 0xc0, 0x93
    ]
def c0 := #[ 1, 17, 14 ]
def c1 := #[ 3, 5, 11, 21, 16, 30, 19 ]
def c2 := #[ 4, 0, 22, 27, 47, 4, 61 ]


def SHIFT := 17
def MINBLOCKLEN := 16
def MAXBLOCKLEN := 64
def MINKEYLEN := 32
def MAXKEYLEN := 128
def KEYLENSTEP := 16

def IsBlockLen x := x % MINBLOCKLEN == 0 && x >= MINBLOCKLEN && x <= MAXBLOCKLEN

def RNDS x :=
    16 + (x - 32) / 16
def ROTL x s :=
    (x <<< s) ||| (x >>> (32 - s))
def ROTL_32 x s :=
    ROTL (UInt32.toNat x) s
def ROTL64 (x: UInt64) s :=
    (x <<< s) ||| (x >>> (64 - s))

structure Kexp_state where
  step : Nat
  s : Nat
deriving Repr

-- I use IO here, since it should be something monadic,
-- but I probably should use other monadic type
def Kexp(key: Array UInt8) (blen: Nat) (rkey: Array UInt8) : Id (Array UInt8) := do
    let mut r0 := mkArray (α := UInt8) 17 0
    let mut r1 := mkArray (α := UInt8) 15 0
    let klen := key.size
    let addk := klen - 32
    let mut step := 0
    let mut s := SHIFT
    let mut rkey_local := rkey
    for i in [0:15-1] do
        r0 := (Array.set! r0 i key[2 * i]!)
        r1 := (Array.set! r1 i key[2 * i + 1]!)

    r0 := Array.set! r0 15 key[30]!
    r0 := Array.set! r0 16 key[31]!
    for r in [0: (RNDS klen - 1)] do
        for k in [0: blen + s - 1] do
            let mut t0 := (sb[UInt8.toNat r0[0]!]! + r0[1]! + sb[UInt8.toNat r0[3]!]! + r0[7]! + sb[UInt8.toNat r0[12]!]! + r0[15]!);
            let mut t1 := (sb[UInt8.toNat r1[0]!]! + r1[3]! + sb[UInt8.toNat r1[9]!]! + r1[12]! + sb[UInt8.toNat r1[14]!]!);
            for i in [0: 14 - 1] do
                r0 := Array.set! r0 i r0[i+1]!
                r1 := Array.set! r1 i r1[i+1]!

            r0 := Array.set! r0 14 r0[15]!
            r0 := Array.set! r0 15 r0[16]!
            if (k >= s) then
                rkey_local := Array.set! rkey_local (r * blen + k - s) (t0 + r1[4]!)
                if (step < addk) then
                    if ((step &&& 1) != 0) then
                        t0 := t0 + key[32 + step]!
                    else
                        t1 := t1 + key[32 + step]!
                    step := step + 1

            r0 := Array.set! r0 16 t0
            r1 := Array.set! r1 14 t1
        s := 0
    return rkey_local

def rot_4_32 x1 x2 x3 x4 :=
    (UInt32.ofNat ((UInt32.toNat x1 ^^^ (ROTL_32 x2 c0[0]) ^^^ (ROTL_32 x3 c0[1]) ^^^ (ROTL_32 x4 c0[2]))))

def rot_8_32 x1 x2 x3 x4 x5 x6 x7 x8 :=
    UInt32.ofNat ((UInt32.toNat x1
        ^^^ (ROTL (UInt32.toNat x2) c1[0])
        ^^^ (ROTL (UInt32.toNat x3) c1[1])
        ^^^ (ROTL (UInt32.toNat x4) c1[2])
        ^^^ (ROTL (UInt32.toNat x5) c1[3])
        ^^^ (ROTL (UInt32.toNat x6) c1[4])
        ^^^ (ROTL (UInt32.toNat x7) c1[5])
        ^^^ (ROTL (UInt32.toNat x8) c1[6])))

def rot_8_64 x1 x2 x3 x4 x5 x6 x7 x8 :=
    UInt64.ofNat ((UInt64.toNat x1
        ^^^ (ROTL (UInt64.toNat x2) c1[0])
        ^^^ (ROTL (UInt64.toNat x3) c1[1])
        ^^^ (ROTL (UInt64.toNat x4) c1[2])
        ^^^ (ROTL (UInt64.toNat x5) c1[3])
        ^^^ (ROTL (UInt64.toNat x6) c1[4])
        ^^^ (ROTL (UInt64.toNat x7) c1[5])
        ^^^ (ROTL (UInt64.toNat x8) c1[6])))

def lin344 (din: Subarray2 UInt32) (dout: Subarray2 UInt32) : Subarray2 UInt32 :=
    let dout := dout.set! 0 (rot_4_32 (din.get! UInt32 0) (din.get! UInt32 1) (din.get! UInt32 2) (din.get! UInt32 3))
    let dout := dout.set! 1 (rot_4_32 (din.get! UInt32 1) (din.get! UInt32 2) (din.get! UInt32 3) (dout.get! UInt32 0))
    let dout := dout.set! 2 (rot_4_32 (din.get! UInt32 2) (din.get! UInt32 3) (dout.get! UInt32 0) (dout.get! UInt32 1))
    let dout := dout.set! 3 (rot_4_32 (din.get! UInt32 3) (dout.get! UInt32 0) (dout.get! UInt32 1) (dout.get! UInt32 2))
    dout

def lin348 (din: Subarray2 UInt32) (dout: Subarray2 UInt32) : Subarray2 UInt32 :=
    let dout := dout.set! 0 (rot_8_32 (din.get! UInt32 0) (din.get! UInt32 1) (din.get! UInt32 2) (din.get! UInt32 3) (din.get! UInt32 4) (din.get! UInt32 5) (din.get! UInt32 6) (din.get! UInt32 7))
    let dout := dout.set! 1 (rot_8_32 (din.get! UInt32 1) (din.get! UInt32 2) (din.get! UInt32 3) (din.get! UInt32 4) (din.get! UInt32 5) (din.get! UInt32 6) (din.get! UInt32 7) (dout.get! UInt32 0))
    let dout := dout.set! 2 (rot_8_32 (din.get! UInt32 2) (din.get! UInt32 3) (din.get! UInt32 4) (din.get! UInt32 5) (din.get! UInt32 6) (din.get! UInt32 7) (dout.get! UInt32 0) (dout.get! UInt32 1))
    let dout := dout.set! 3 (rot_8_32 (din.get! UInt32 3) (din.get! UInt32 4) (din.get! UInt32 5) (din.get! UInt32 6) (din.get! UInt32 7) (dout.get! UInt32 0) (dout.get! UInt32 1) (dout.get! UInt32 2))
    let dout := dout.set! 4 (rot_8_32 (din.get! UInt32 4) (din.get! UInt32 5) (din.get! UInt32 6) (din.get! UInt32 7) (dout.get! UInt32 0) (dout.get! UInt32 1) (dout.get! UInt32 2) (dout.get! UInt32 3))
    let dout := dout.set! 5 (rot_8_32 (din.get! UInt32 5) (din.get! UInt32 6) (din.get! UInt32 7) (dout.get! UInt32 0) (dout.get! UInt32 1) (dout.get! UInt32 2) (dout.get! UInt32 3) (dout.get! UInt32 4))
    let dout := dout.set! 6 (rot_8_32 (din.get! UInt32 6) (din.get! UInt32 7) (dout.get! UInt32 0) (dout.get! UInt32 1) (dout.get! UInt32 2) (dout.get! UInt32 3) (dout.get! UInt32 4) (dout.get! UInt32 5))
    let dout := dout.set! 7 (rot_8_32 (din.get! UInt32 7) (dout.get! UInt32 0) (dout.get! UInt32 1) (dout.get! UInt32 2) (dout.get! UInt32 3) (dout.get! UInt32 4) (dout.get! UInt32 5) (dout.get! UInt32 6))
    dout

def lin388 (din: Subarray2 UInt64) (dout: Subarray2 UInt64) : Subarray2 UInt64 :=
    let dout := dout.set! 0 (rot_8_64 (din.get! UInt64 0) (din.get! UInt64 1) (din.get! UInt64 2) (din.get! UInt64 3) (din.get! UInt64 4) (din.get! UInt64 5) (din.get! UInt64 6) (din.get! UInt64 7))
    let dout := dout.set! 1 (rot_8_64 (din.get! UInt64 1) (din.get! UInt64 2) (din.get! UInt64 3) (din.get! UInt64 4) (din.get! UInt64 5) (din.get! UInt64 6) (din.get! UInt64 7) (dout.get! UInt64 0))
    let dout := dout.set! 2 (rot_8_64 (din.get! UInt64 2) (din.get! UInt64 3) (din.get! UInt64 4) (din.get! UInt64 5) (din.get! UInt64 6) (din.get! UInt64 7) (dout.get! UInt64 0) (dout.get! UInt64 1))
    let dout := dout.set! 3 (rot_8_64 (din.get! UInt64 3) (din.get! UInt64 4) (din.get! UInt64 5) (din.get! UInt64 6) (din.get! UInt64 7) (dout.get! UInt64 0) (dout.get! UInt64 1) (dout.get! UInt64 2))
    let dout := dout.set! 4 (rot_8_64 (din.get! UInt64 4) (din.get! UInt64 5) (din.get! UInt64 6) (din.get! UInt64 7) (dout.get! UInt64 0) (dout.get! UInt64 1) (dout.get! UInt64 2) (dout.get! UInt64 3))
    let dout := dout.set! 5 (rot_8_64 (din.get! UInt64 5) (din.get! UInt64 6) (din.get! UInt64 7) (dout.get! UInt64 0) (dout.get! UInt64 1) (dout.get! UInt64 2) (dout.get! UInt64 3) (dout.get! UInt64 4))
    let dout := dout.set! 6 (rot_8_64 (din.get! UInt64 6) (din.get! UInt64 7) (dout.get! UInt64 0) (dout.get! UInt64 1) (dout.get! UInt64 2) (dout.get! UInt64 3) (dout.get! UInt64 4) (dout.get! UInt64 5))
    let dout := dout.set! 7 (rot_8_64 (din.get! UInt64 7) (dout.get! UInt64 0) (dout.get! UInt64 1) (dout.get! UInt64 2) (dout.get! UInt64 3) (dout.get! UInt64 4) (dout.get! UInt64 5) (dout.get! UInt64 6))
    dout

-- def linOp (d: Subarray2 UInt8) (r: Subarray2 UInt8) (blocklen: Nat) :=
--     match blocklen with
--     | 16 => lin344 d r
--     | 32 => lin384 d r
--     | 64 => lin388 d r
--     | _ => sorry
