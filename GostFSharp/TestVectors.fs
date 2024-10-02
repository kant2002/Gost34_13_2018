module TestVectors

open Qalqan
open PRNG
open System.IO
open Microsoft.FSharp.NativeInterop

#nowarn 9
let Pr3(b: nativeptr<byte>, len, f: StreamWriter) =
    for i = 0 to len - 1 do
        f.Write("{0:x2}", NativePtr.get b i);
    f.Write("\n")

let Pr(str: System.String, b: nativeptr<byte>, len, f: StreamWriter) =
    f.Write("{0}:\n", str)
    for i = 0 to len - 1 do
        f.Write("{0:x2}", NativePtr.get b i);
    f.Write("\n")

let KexpV(key: nativeptr<byte>, klen: int, blen: int, rkey: nativeptr<byte>, f: StreamWriter) =
    Pr("Initial key", key, klen, f)
    let r0 = NativePtr.stackalloc<byte> 17
    let r1 = NativePtr.stackalloc<byte> 15
    let addk = klen - 32
    let mutable step = 0
    let mutable s = SHIFT
    for i = 0 to 15 - 1 do
        NativePtr.set r0 i (NativePtr.get key (2 * i))
        NativePtr.set r1 i (NativePtr.get key (2 * i + 1))
        
    NativePtr.set r0 15 (NativePtr.get key 30)
    NativePtr.set r0 16 (NativePtr.get key 31)
    Pr("Register L0", r0, 17, f)
    Pr("Register L1", r1, 15, f)
    for r = 0 to RNDS(klen) - 1 do
        for k = 0 to blen + s - 1 do
            let mutable t0 = (sb[int (NativePtr.get r0 0)] + (NativePtr.get r0 1) + sb[int (NativePtr.get r0 3)] + (NativePtr.get r0 7) + sb[int (NativePtr.get r0 12)] + (NativePtr.get r0 16));
            let mutable t1 = (sb[int (NativePtr.get r1 0)] + (NativePtr.get r1 3) + sb[int (NativePtr.get r1 9)] + (NativePtr.get r1 12) + sb[int (NativePtr.get r1 14)]);
            for i = 0 to 14 - 1 do
                NativePtr.set r0 i (NativePtr.get r0 (i+1))
                NativePtr.set r1 i (NativePtr.get r1 (i+1))

            NativePtr.set r0 14 (NativePtr.get r0 15)
            NativePtr.set r0 15 (NativePtr.get r0 16)
            if (k >= s) then
                NativePtr.set rkey (r * blen + k - s) (t0 + NativePtr.get r1 4)
                if (step < addk) then
                    if ((step &&& 1) <> 0) then
                        //f.Write("Additional key byte to L0: {0:x2}\n",NativePtr.get key (32 + step))
                        t0 <- t0 + NativePtr.get key (32 + step)
                    else
                        //f.Write("Additional key byte to L1: {0:x2}\n",NativePtr.get key (32 + step))
                        t1 <- t1 + NativePtr.get key (32 + step)
                    step <- step + 1
                    
                //if (r < 2) then
                //    f.Write("Output {0:x2} (L0: {1:x2}, L1: {2:x2})\n", (t0 + NativePtr.get r1 4) &&& 0xffuy, t0, NativePtr.get r1 4);
            NativePtr.set r0 16 t0
            NativePtr.set r1 14 t1
            //if (step < addk || (!r && (k < s))) then
            //    f.Write("Step {0}\nFeedback L0: {1:x2}, L1: {2:x2}\n", k, t0, t1);
            //    Pr("Register L0", r0, 17, f);
            //    Pr("Register L1", r1, 15, f);
        if (s <> 0) then
            f.Write("After prerun:\n")
            Pr("Register L0", r0, 17, f)
            Pr("Register L1", r1, 15, f)

        s <- 0
    
    for r = 0 to RNDS(klen) - 1 do
        f.Write("Round {0,2} key: ", r)
        Pr3(NativePtr.add rkey (blen * r), blen, f)

    ()

let KexpVV(key: nativeptr<byte>, klen: int, blen: int, rkey: nativeptr<byte>, f: StreamWriter) =
    Pr("Initial key", key, klen, f)
    let r0 = NativePtr.stackalloc<byte> 17
    let r1 = NativePtr.stackalloc<byte> 15
    let addk = klen - 32
    let mutable step = 0
    let mutable s = SHIFT
    for i = 0 to 15 - 1 do
        NativePtr.set r0 i (NativePtr.get key (2 * i))
        NativePtr.set r1 i (NativePtr.get key (2 * i + 1))
        
    NativePtr.set r0 15 (NativePtr.get key 30)
    NativePtr.set r0 16 (NativePtr.get key 31)
    Pr("Register L0", r0, 17, f)
    Pr("Register L1", r1, 15, f)
    for r = 0 to RNDS(klen) - 1 do
        for k = 0 to blen + s - 1 do
            let mutable t0 = (sb[int (NativePtr.get r0 0)] + (NativePtr.get r0 1) + sb[int (NativePtr.get r0 3)] + (NativePtr.get r0 7) + sb[int (NativePtr.get r0 12)] + (NativePtr.get r0 16));
            let mutable t1 = (sb[int (NativePtr.get r1 0)] + (NativePtr.get r1 3) + sb[int (NativePtr.get r1 9)] + (NativePtr.get r1 12) + sb[int (NativePtr.get r1 14)]);
            for i = 0 to 14 - 1 do
                NativePtr.set r0 i (NativePtr.get r0 (i+1))
                NativePtr.set r1 i (NativePtr.get r1 (i+1))

            NativePtr.set r0 14 (NativePtr.get r0 15)
            NativePtr.set r0 15 (NativePtr.get r0 16)
            if (k >= s) then
                NativePtr.set rkey (r * blen + k - s) (t0 + NativePtr.get r1 4)
                if (step < addk) then
                    if ((step &&& 1) <> 0) then
                        if (step < 32) then
                            f.Write("Additional key byte to L0: {0:x2}\n",NativePtr.get key (32 + step))
                        t0 <- t0 + NativePtr.get key (32 + step)
                    else
                        if (step < 32) then
                            f.Write("Additional key byte to L1: {0:x2}\n",NativePtr.get key (32 + step))
                        t1 <- t1 + NativePtr.get key (32 + step)
                    if (step < 32) then
                        Pr("Register L0", r0, 17, f)
                        Pr("Register L1", r1, 15, f)

                    step <- step + 1
                    
                if (step < 8) then
                    f.Write("Output {0:x2} (L0: {1:x2}, L1: {2:x2})\n", (t0 + NativePtr.get r1 4) &&& 0xffuy, t0, NativePtr.get r1 4);
            NativePtr.set r0 16 t0
            NativePtr.set r1 14 t1
            //if (step < addk || (!r && (k < s)))
            if (step < 16) then
                f.Write("Step {0}\nFeedback L0: {1:x2}, L1: {2:x2}\n", k, t0, t1);
                Pr("Register L0", r0, 17, f);
                Pr("Register L1", r1, 15, f);
        s <- 0
    
    for r = 0 to RNDS(klen) - 1 do
        f.Write("Round {0,2} key: ", r)
        Pr3(NativePtr.add rkey (blen * r), blen, f)

    ()

let encryptV(data: nativeptr<byte>, rkey: nativeptr<byte>, klen, blen, res: nativeptr<byte>, f: StreamWriter) =
    let block = NativePtr.stackalloc<byte> MAXBLOCKLEN
    let block2 = NativePtr.stackalloc<byte> MAXBLOCKLEN
    Pr("Clear text", data, blen, f);
    Pr("Key 0", rkey, blen, f);
    AddRk(data, rkey, 0, blen, block)
    Pr("After add K0", block, blen, f)
    sBox(block, block2, blen)
    Pr("After Sub", block2, blen, f)
    linOp(block2, block, blen)
    Pr("After linear", block, blen, f)
    for i = 1 to RNDS(klen) - 2 do
        f.Write("Round {0}.\nKey{0}:\n", i, i);
        Pr3(NativePtr.add rkey (blen * i), blen, f);
        AddRkX(block, rkey, i, blen, block2)
        Pr("After addkey", block2, blen, f)
        sBox(block2, block2, blen)
        Pr("After Sub", block2, blen, f)
        linOp(block2, block, blen)
        Pr("After linear", block, blen, f)

    Pr("Final key", NativePtr.add rkey (blen * (RNDS(klen) - 1)), blen, f)
    AddRk(block, rkey, RNDS(klen) - 1, blen, res)
    Pr("Ciphertext", res, blen, f)

let fread_s(din: nativeptr<byte>, bufferSize, size, count, src: FileStream) =
    for i = 0 to count - 1 do
        NativePtr.set din i (byte (src.ReadByte()))
    
[<Literal>]
let BLEN_CNT = 3

let ShortTestLin (f: StreamWriter, src: FileStream) =
    let din = NativePtr.stackalloc<byte> MAXBLOCKLEN
    let dout = NativePtr.stackalloc<byte> MAXBLOCKLEN
    if (src = null) then
        for i = 0 to MAXBLOCKLEN - 1 do
            NativePtr.set din i (rnext())
    else
        fread_s(din, MAXBLOCKLEN, sizeof<byte>, MAXBLOCKLEN, src)

    let blen_vals = [| 16; 32; 64 |]
    f.Write("Linear operation:\n");
    for bl in blen_vals do

        f.Write("Input ({0} bits):  ", bl * 8);
        for i = 0 to bl - 1 do
            f.Write("{0:x2}", NativePtr.get din i);
        f.Write("\nOutput ({0} bits): ", bl * 8);
        linOp(din, dout, bl)
        for i = 0 to bl - 1 do
            f.Write("{0:x2}", NativePtr.get dout i)
        f.Write("\n")

    f.Write("\n")

let ShortTestSBox (f: StreamWriter, src: FileStream) =
    let din = NativePtr.stackalloc<byte> MAXBLOCKLEN
    let dout = NativePtr.stackalloc<byte> MAXBLOCKLEN
    if (src = null) then
        for i = 0 to MAXBLOCKLEN - 1 do
            NativePtr.set din i (rnext())
            NativePtr.set dout i sb[int (NativePtr.get din i)]
    else
        fread_s(din, MAXBLOCKLEN, sizeof<byte>, MAXBLOCKLEN, src);
        for i = 0 to MAXBLOCKLEN - 1 do
            NativePtr.set dout i sb[int (NativePtr.get din i)]
    
    f.Write("Nonlinear operation:\n")
    f.Write("Input ({0} bits):  ", MAXBLOCKLEN * 8)
    for i = 0 to MAXBLOCKLEN - 1 do
        f.Write("{0:x2}", NativePtr.get din i);
    f.Write("\nOutput ({0} bits): ", MAXBLOCKLEN * 8)
    for i = 0 to MAXBLOCKLEN - 1 do
        f.Write("{0:x2}", NativePtr.get dout i)
    f.Write("\n\n")

let ShortTestKExp (f: StreamWriter, src: FileStream) =
    let key = NativePtr.stackalloc<byte> MAXKEYLEN
    let rkey = NativePtr.stackalloc<byte> (RNDS(MAXKEYLEN) * MAXBLOCKLEN)
    if (src = null) then
        for i = 0 to MAXKEYLEN - 1 do
            NativePtr.set key i (rnext())
    else
        fread_s(key, MAXKEYLEN, sizeof<byte>, MAXKEYLEN, src)
    
    f.Write("\nKey expansion for {0} bit key and {1} bit block:\n", MAXKEYLEN * 8, MAXBLOCKLEN * 8)
    KexpVV(key, MAXKEYLEN, MAXBLOCKLEN, rkey, f)
    f.Write("\n")

let ShortTestBasicEnc (f: StreamWriter, src: FileStream) =
    let key = NativePtr.stackalloc<byte> MAXKEYLEN
    let rkey = NativePtr.stackalloc<byte> (RNDS(MAXKEYLEN) * MAXBLOCKLEN)
    let data = NativePtr.stackalloc<byte> MINBLOCKLEN
    let cipher = NativePtr.stackalloc<byte> MINBLOCKLEN
    if (src = null) then
        for i = 0 to MINBLOCKLEN - 1 do
            NativePtr.set data i (rnext())
        for i = 0 to MINKEYLEN - 1 do
            NativePtr.set key i (rnext())
    else
        fread_s(data, MINBLOCKLEN, sizeof<byte>, MINBLOCKLEN, src);
        fread_s(key, MAXKEYLEN, sizeof<byte>, MAXKEYLEN, src)
    
    f.Write("Encryption of 128 bit block and 256 bit key\n")
    Kexp(key, MINKEYLEN, MINBLOCKLEN, rkey)
    encryptV(data, rkey, MINKEYLEN, MINBLOCKLEN, cipher, f)
    f.Write("\n")

let ShortTestEnc (f: StreamWriter, src: FileStream) =
    let key = NativePtr.stackalloc<byte> MAXKEYLEN
    let rkey = NativePtr.stackalloc<byte> (RNDS(MAXKEYLEN) * MAXBLOCKLEN)
    let data = NativePtr.stackalloc<byte> MINBLOCKLEN
    let cipher = NativePtr.stackalloc<byte> MINBLOCKLEN
    if (src = null) then
        for i = 0 to MAXBLOCKLEN - 1 do
            NativePtr.set data i (rnext())
        for i = 0 to MAXKEYLEN - 1 do
            NativePtr.set key i (rnext())
    else
        fread_s(data, MAXBLOCKLEN, sizeof<byte>, MAXBLOCKLEN, src)
        fread_s(key, MAXKEYLEN, sizeof<byte>, MAXKEYLEN, src)
    
    let mutable klen = MINKEYLEN
    while klen <= MAXKEYLEN do
        f.Write("\n***** Key expansion, key length = {0}, block length = {1} *****\n", klen * 8, MINBLOCKLEN * 8)
        KexpV(key, klen, MINBLOCKLEN, rkey, f)
        f.Write("\n")
        klen <- klen + KEYLENSTEP

    let blen_vals = [| 16; 32; 64 |]
    for blen in blen_vals do
        let data2 = NativePtr.stackalloc<byte> MAXBLOCKLEN
        let cipher2 = NativePtr.stackalloc<byte> MAXBLOCKLEN
        Kexp(key, MINKEYLEN, blen, rkey)
        f.Write("\n***** Encryption, key length = {0}, block len = {1} *****\n", MINKEYLEN * 8, blen * 8)
        encryptV(data2, rkey, MINKEYLEN, blen, cipher2, f)
        f.Write("\n")

let ShortTestVectors (f: StreamWriter, src: FileStream) =
    ShortTestLin(f, src);
    ShortTestLin(f, src);
    ShortTestSBox(f, src);
    ShortTestKExp(f, src);
    //ShortTestBasicEnc(f);
    ShortTestEnc(f, src);
