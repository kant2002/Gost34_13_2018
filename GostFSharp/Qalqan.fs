module Qalqan

open Microsoft.FSharp.NativeInterop

let sb : byte[] = 
    [| (* ded: OK, dif: 4, dip: 7, lin: 32, pow: 7, cor: 0, dst: 112, sac: 116..140, cyc: 256 *)
        0xd1uy; 0xb5uy; 0xa6uy; 0x74uy; 0x2fuy; 0xb2uy; 0x03uy; 0x77uy; 0xaeuy; 0xb3uy; 0x60uy; 0x95uy; 0xfduy; 0xf8uy; 0xc7uy; 0xf0uy;
        0x2buy; 0xceuy; 0xa5uy; 0x91uy; 0x4cuy; 0x6fuy; 0xf3uy; 0x4fuy; 0x82uy; 0x01uy; 0x45uy; 0x76uy; 0x9fuy; 0xeduy; 0x41uy; 0xfbuy;
        0xacuy; 0x4euy; 0x5euy; 0x04uy; 0xebuy; 0xf9uy; 0xf1uy; 0x3auy; 0x1fuy; 0xe2uy; 0x8euy; 0xe7uy; 0x85uy; 0x35uy; 0xdbuy; 0x52uy;
        0x78uy; 0xa1uy; 0xfcuy; 0xa2uy; 0xdeuy; 0x68uy; 0x02uy; 0x4duy; 0xf6uy; 0xdduy; 0xcfuy; 0xa3uy; 0xdcuy; 0x6buy; 0x81uy; 0x44uy;
        0x2auy; 0x5duy; 0x1euy; 0xe0uy; 0x53uy; 0x71uy; 0x3buy; 0xc1uy; 0xccuy; 0x9duy; 0x80uy; 0xd5uy; 0x84uy; 0x00uy; 0x24uy; 0x4buy;
        0xb6uy; 0x83uy; 0x0duy; 0x87uy; 0x7euy; 0x86uy; 0xcauy; 0x96uy; 0xbeuy; 0x5auy; 0xe6uy; 0xd0uy; 0xd4uy; 0xd8uy; 0x55uy; 0xc0uy;
        0x05uy; 0xe5uy; 0xe9uy; 0x5buy; 0x47uy; 0xe4uy; 0x2duy; 0x34uy; 0x13uy; 0x88uy; 0x48uy; 0x32uy; 0x38uy; 0xb9uy; 0xdauy; 0xc9uy;
        0x42uy; 0x29uy; 0xd7uy; 0xf2uy; 0x9buy; 0x6duy; 0xe8uy; 0x8duy; 0x12uy; 0x7cuy; 0x8cuy; 0x3fuy; 0xbcuy; 0x3cuy; 0x1buy; 0xc5uy;
        0x69uy; 0x22uy; 0x97uy; 0xaauy; 0x73uy; 0x0auy; 0x0cuy; 0x8auy; 0x90uy; 0x31uy; 0xc4uy; 0x33uy; 0xe1uy; 0x8buy; 0x9cuy; 0x63uy;
        0x5fuy; 0xf5uy; 0xf7uy; 0xffuy; 0x79uy; 0x49uy; 0xd3uy; 0xc6uy; 0x7buy; 0x1auy; 0x39uy; 0xc8uy; 0x6euy; 0x72uy; 0xd9uy; 0xc3uy;
        0x62uy; 0x28uy; 0xbduy; 0xbbuy; 0xfauy; 0x2euy; 0xbfuy; 0x43uy; 0x06uy; 0x0buy; 0x7auy; 0x64uy; 0x5cuy; 0x92uy; 0x37uy; 0x3duy;
        0x66uy; 0x26uy; 0x51uy; 0xefuy; 0x0fuy; 0xa9uy; 0x14uy; 0x70uy; 0x16uy; 0x17uy; 0x10uy; 0x19uy; 0x93uy; 0x09uy; 0x59uy; 0x15uy;
        0xfeuy; 0x4auy; 0xcbuy; 0x2cuy; 0xcduy; 0xb8uy; 0x94uy; 0xabuy; 0xdfuy; 0xa7uy; 0x0euy; 0x30uy; 0xafuy; 0x56uy; 0x23uy; 0xb1uy;
        0xb0uy; 0x58uy; 0x7duy; 0xc2uy; 0x1duy; 0x50uy; 0x20uy; 0x61uy; 0x25uy; 0x89uy; 0xa0uy; 0x6cuy; 0x11uy; 0x54uy; 0x98uy; 0xb7uy;
        0x18uy; 0x21uy; 0xaduy; 0x3euy; 0xd2uy; 0xeauy; 0x40uy; 0xd6uy; 0xf4uy; 0xa4uy; 0x8fuy; 0xa8uy; 0x08uy; 0x57uy; 0xbauy; 0xeeuy;
        0x75uy; 0x6auy; 0x07uy; 0x99uy; 0x7fuy; 0x1cuy; 0xe3uy; 0x46uy; 0x67uy; 0xecuy; 0x27uy; 0x36uy; 0xb4uy; 0x65uy; 0x9euy; 0x9auy
    |]
let isb : byte[] = 
    [|
        0x4duy; 0x19uy; 0x36uy; 0x06uy; 0x23uy; 0x60uy; 0xa8uy; 0xf2uy; 0xecuy; 0xbduy; 0x85uy; 0xa9uy; 0x86uy; 0x52uy; 0xcauy; 0xb4uy;
        0xbauy; 0xdcuy; 0x78uy; 0x68uy; 0xb6uy; 0xbfuy; 0xb8uy; 0xb9uy; 0xe0uy; 0xbbuy; 0x99uy; 0x7euy; 0xf5uy; 0xd4uy; 0x42uy; 0x28uy;
        0xd6uy; 0xe1uy; 0x81uy; 0xceuy; 0x4euy; 0xd8uy; 0xb1uy; 0xfauy; 0xa1uy; 0x71uy; 0x40uy; 0x10uy; 0xc3uy; 0x66uy; 0xa5uy; 0x04uy;
        0xcbuy; 0x89uy; 0x6buy; 0x8buy; 0x67uy; 0x2duy; 0xfbuy; 0xaeuy; 0x6cuy; 0x9auy; 0x27uy; 0x46uy; 0x7duy; 0xafuy; 0xe3uy; 0x7buy;
        0xe6uy; 0x1euy; 0x70uy; 0xa7uy; 0x3fuy; 0x1auy; 0xf7uy; 0x64uy; 0x6auy; 0x95uy; 0xc1uy; 0x4fuy; 0x14uy; 0x37uy; 0x21uy; 0x17uy;
        0xd5uy; 0xb2uy; 0x2fuy; 0x44uy; 0xdduy; 0x5euy; 0xcduy; 0xeduy; 0xd1uy; 0xbeuy; 0x59uy; 0x63uy; 0xacuy; 0x41uy; 0x22uy; 0x90uy;
        0x0auy; 0xd7uy; 0xa0uy; 0x8fuy; 0xabuy; 0xfduy; 0xb0uy; 0xf8uy; 0x35uy; 0x80uy; 0xf1uy; 0x3duy; 0xdbuy; 0x75uy; 0x9cuy; 0x15uy;
        0xb7uy; 0x45uy; 0x9duy; 0x84uy; 0x03uy; 0xf0uy; 0x1buy; 0x07uy; 0x30uy; 0x94uy; 0xaauy; 0x98uy; 0x79uy; 0xd2uy; 0x54uy; 0xf4uy;
        0x4auy; 0x3euy; 0x18uy; 0x51uy; 0x4cuy; 0x2cuy; 0x55uy; 0x53uy; 0x69uy; 0xd9uy; 0x87uy; 0x8duy; 0x7auy; 0x77uy; 0x2auy; 0xeauy;
        0x88uy; 0x13uy; 0xaduy; 0xbcuy; 0xc6uy; 0x0buy; 0x57uy; 0x82uy; 0xdeuy; 0xf3uy; 0xffuy; 0x74uy; 0x8euy; 0x49uy; 0xfeuy; 0x1cuy;
        0xdauy; 0x31uy; 0x33uy; 0x3buy; 0xe9uy; 0x12uy; 0x02uy; 0xc9uy; 0xebuy; 0xb5uy; 0x83uy; 0xc7uy; 0x20uy; 0xe2uy; 0x08uy; 0xccuy;
        0xd0uy; 0xcfuy; 0x05uy; 0x09uy; 0xfcuy; 0x01uy; 0x50uy; 0xdfuy; 0xc5uy; 0x6duy; 0xeeuy; 0xa3uy; 0x7cuy; 0xa2uy; 0x58uy; 0xa6uy;
        0x5fuy; 0x47uy; 0xd3uy; 0x9fuy; 0x8auy; 0x7fuy; 0x97uy; 0x0euy; 0x9buy; 0x6fuy; 0x56uy; 0xc2uy; 0x48uy; 0xc4uy; 0x11uy; 0x3auy;
        0x5buy; 0x00uy; 0xe4uy; 0x96uy; 0x5cuy; 0x4buy; 0xe7uy; 0x72uy; 0x5duy; 0x9euy; 0x6euy; 0x2euy; 0x3cuy; 0x39uy; 0x34uy; 0xc8uy;
        0x43uy; 0x8cuy; 0x29uy; 0xf6uy; 0x65uy; 0x61uy; 0x5auy; 0x2buy; 0x76uy; 0x62uy; 0xe5uy; 0x24uy; 0xf9uy; 0x1duy; 0xefuy; 0xb3uy;
        0x0fuy; 0x26uy; 0x73uy; 0x16uy; 0xe8uy; 0x91uy; 0x38uy; 0x92uy; 0x0duy; 0x25uy; 0xa4uy; 0x1fuy; 0x32uy; 0x0cuy; 0xc0uy; 0x93uy;
    |]

let c0 = [| 1; 17; 14 |]
let c1 = [| 3; 5; 11; 21; 16; 30; 19 |]
let c2 = [| 4; 0; 22; 27; 47; 4; 61 |]

[<Literal>]
let SHIFT = 17
[<Literal>]
let MINBLOCKLEN = 16
[<Literal>]
let MAXBLOCKLEN = 64
[<Literal>]
let MINKEYLEN = 32
[<Literal>]
let MAXKEYLEN = 128
[<Literal>]
let KEYLENSTEP = 16

let RNDS x = 
    16 + (x - 32) / 16
let ROTL (x, s) = 
    (x <<< s) ||| (x >>> (32 - s))
let ROTL64 (x: uint64, s) = 
    (x <<< s) ||| (x >>> (64 - s))

#nowarn 9
let Kexp(key: nativeptr<byte>, klen: int, blen: int, rkey: nativeptr<byte>) =
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
                        t0 <- t0 + NativePtr.get key (32 + step)
                    else
                        t1 <- t1 + NativePtr.get key (32 + step)
                    step <- step + 1

            NativePtr.set r0 16 t0
            NativePtr.set r1 14 t1
        s <- 0
    ()

let lin344 (din: nativeptr<uint>, dout: nativeptr<uint>) =
    NativePtr.set dout 0 ((NativePtr.get din 0) ^^^ ROTL((NativePtr.get din 1), c0[0]) ^^^ ROTL((NativePtr.get din 2), c0[1]) ^^^ ROTL((NativePtr.get din 3), c0[2]))
    NativePtr.set dout 1 ((NativePtr.get din 1) ^^^ ROTL((NativePtr.get din 2), c0[0]) ^^^ ROTL((NativePtr.get din 3), c0[1]) ^^^ ROTL((NativePtr.get dout 0), c0[2]))
    NativePtr.set dout 2 ((NativePtr.get din 2) ^^^ ROTL((NativePtr.get din 3), c0[0]) ^^^ ROTL((NativePtr.get dout 0), c0[1]) ^^^ ROTL((NativePtr.get dout 1), c0[2]))
    NativePtr.set dout 3 ((NativePtr.get din 3) ^^^ ROTL((NativePtr.get dout 0), c0[0]) ^^^ ROTL((NativePtr.get dout 1), c0[1]) ^^^ ROTL((NativePtr.get dout 2), c0[2]))

let lin384 (din: nativeptr<uint>, dout: nativeptr<uint>) =
    NativePtr.set dout 0 ((NativePtr.get din 0) ^^^ ROTL((NativePtr.get din 1), c1[0]) ^^^ ROTL((NativePtr.get din 2), c1[1]) ^^^ ROTL((NativePtr.get din 3), c1[2]) ^^^ ROTL((NativePtr.get din 4), c1[3]) ^^^ ROTL((NativePtr.get din 5), c1[4]) ^^^ ROTL((NativePtr.get din 6), c1[5]) ^^^ ROTL((NativePtr.get din 7), c1[6]))
    NativePtr.set dout 1 ((NativePtr.get din 1) ^^^ ROTL((NativePtr.get din 2), c1[0]) ^^^ ROTL((NativePtr.get din 3), c1[1]) ^^^ ROTL((NativePtr.get din 4), c1[2]) ^^^ ROTL((NativePtr.get din 5), c1[3]) ^^^ ROTL((NativePtr.get din 6), c1[4]) ^^^ ROTL((NativePtr.get din 7), c1[5]) ^^^ ROTL((NativePtr.get dout 0), c1[6]))
    NativePtr.set dout 2 ((NativePtr.get din 2) ^^^ ROTL((NativePtr.get din 3), c1[0]) ^^^ ROTL((NativePtr.get din 4), c1[1]) ^^^ ROTL((NativePtr.get din 5), c1[2]) ^^^ ROTL((NativePtr.get din 6), c1[3]) ^^^ ROTL((NativePtr.get din 7), c1[4]) ^^^ ROTL((NativePtr.get dout 0), c1[5]) ^^^ ROTL((NativePtr.get dout 1), c1[6]))
    NativePtr.set dout 3 ((NativePtr.get din 3) ^^^ ROTL((NativePtr.get din 4), c1[0]) ^^^ ROTL((NativePtr.get din 5), c1[1]) ^^^ ROTL((NativePtr.get din 6), c1[2]) ^^^ ROTL((NativePtr.get din 7), c1[3]) ^^^ ROTL((NativePtr.get dout 0), c1[4]) ^^^ ROTL((NativePtr.get dout 1), c1[5]) ^^^ ROTL((NativePtr.get dout 2), c1[6]))
    NativePtr.set dout 4 ((NativePtr.get din 4) ^^^ ROTL((NativePtr.get din 5), c1[0]) ^^^ ROTL((NativePtr.get din 6), c1[1]) ^^^ ROTL((NativePtr.get din 7), c1[2]) ^^^ ROTL((NativePtr.get dout 0), c1[3]) ^^^ ROTL((NativePtr.get dout 1), c1[4]) ^^^ ROTL((NativePtr.get dout 2), c1[5]) ^^^ ROTL((NativePtr.get dout 3), c1[6]))
    NativePtr.set dout 5 ((NativePtr.get din 5) ^^^ ROTL((NativePtr.get din 6), c1[0]) ^^^ ROTL((NativePtr.get din 7), c1[1]) ^^^ ROTL((NativePtr.get dout 0), c1[2]) ^^^ ROTL((NativePtr.get dout 1), c1[3]) ^^^ ROTL((NativePtr.get dout 2), c1[4]) ^^^ ROTL((NativePtr.get dout 3), c1[5]) ^^^ ROTL((NativePtr.get dout 4), c1[6]))
    NativePtr.set dout 6 ((NativePtr.get din 6) ^^^ ROTL((NativePtr.get din 7), c1[0]) ^^^ ROTL((NativePtr.get dout 0), c1[1]) ^^^ ROTL((NativePtr.get dout 1), c1[2]) ^^^ ROTL((NativePtr.get dout 2), c1[3]) ^^^ ROTL((NativePtr.get dout 3), c1[4]) ^^^ ROTL((NativePtr.get dout 4), c1[5]) ^^^ ROTL((NativePtr.get dout 5), c1[6]))
    NativePtr.set dout 7 ((NativePtr.get din 7) ^^^ ROTL((NativePtr.get dout 0), c1[0]) ^^^ ROTL((NativePtr.get dout 1), c1[1]) ^^^ ROTL((NativePtr.get dout 2), c1[2]) ^^^ ROTL((NativePtr.get dout 3), c1[3]) ^^^ ROTL((NativePtr.get dout 4), c1[4]) ^^^ ROTL((NativePtr.get dout 5), c1[5]) ^^^ ROTL((NativePtr.get dout 6), c1[6]))

let lin388 (din: nativeptr<uint64>, dout: nativeptr<uint64>) =
    NativePtr.set dout 0 ((NativePtr.get din 0) ^^^ ROTL64((NativePtr.get din 1), c2[0]) ^^^ ROTL64((NativePtr.get din 2), c2[1]) ^^^ ROTL64((NativePtr.get din 3), c2[2]) ^^^ ROTL64((NativePtr.get din 4), c2[3]) ^^^ ROTL64((NativePtr.get din 5), c2[4]) ^^^ ROTL64((NativePtr.get din 6), c2[5]) ^^^ ROTL64((NativePtr.get din 7), c2[6]))
    NativePtr.set dout 1 ((NativePtr.get din 1) ^^^ ROTL64((NativePtr.get din 2), c2[0]) ^^^ ROTL64((NativePtr.get din 3), c2[1]) ^^^ ROTL64((NativePtr.get din 4), c2[2]) ^^^ ROTL64((NativePtr.get din 5), c2[3]) ^^^ ROTL64((NativePtr.get din 6), c2[4]) ^^^ ROTL64((NativePtr.get din 7), c2[5]) ^^^ ROTL64((NativePtr.get dout 0), c2[6]))
    NativePtr.set dout 2 ((NativePtr.get din 2) ^^^ ROTL64((NativePtr.get din 3), c2[0]) ^^^ ROTL64((NativePtr.get din 4), c2[1]) ^^^ ROTL64((NativePtr.get din 5), c2[2]) ^^^ ROTL64((NativePtr.get din 6), c2[3]) ^^^ ROTL64((NativePtr.get din 7), c2[4]) ^^^ ROTL64((NativePtr.get dout 0), c2[5]) ^^^ ROTL64((NativePtr.get dout 1), c2[6]))
    NativePtr.set dout 3 ((NativePtr.get din 3) ^^^ ROTL64((NativePtr.get din 4), c2[0]) ^^^ ROTL64((NativePtr.get din 5), c2[1]) ^^^ ROTL64((NativePtr.get din 6), c2[2]) ^^^ ROTL64((NativePtr.get din 7), c2[3]) ^^^ ROTL64((NativePtr.get dout 0), c2[4]) ^^^ ROTL64((NativePtr.get dout 1), c2[5]) ^^^ ROTL64((NativePtr.get dout 2), c2[6]))
    NativePtr.set dout 4 ((NativePtr.get din 4) ^^^ ROTL64((NativePtr.get din 5), c2[0]) ^^^ ROTL64((NativePtr.get din 6), c2[1]) ^^^ ROTL64((NativePtr.get din 7), c2[2]) ^^^ ROTL64((NativePtr.get dout 0), c2[3]) ^^^ ROTL64((NativePtr.get dout 1), c2[4]) ^^^ ROTL64((NativePtr.get dout 2), c2[5]) ^^^ ROTL64((NativePtr.get dout 3), c2[6]))
    NativePtr.set dout 5 ((NativePtr.get din 5) ^^^ ROTL64((NativePtr.get din 6), c2[0]) ^^^ ROTL64((NativePtr.get din 7), c2[1]) ^^^ ROTL64((NativePtr.get dout 0), c2[2]) ^^^ ROTL64((NativePtr.get dout 1), c2[3]) ^^^ ROTL64((NativePtr.get dout 2), c2[4]) ^^^ ROTL64((NativePtr.get dout 3), c2[5]) ^^^ ROTL64((NativePtr.get dout 4), c2[6]))
    NativePtr.set dout 6 ((NativePtr.get din 6) ^^^ ROTL64((NativePtr.get din 7), c2[0]) ^^^ ROTL64((NativePtr.get dout 0), c2[1]) ^^^ ROTL64((NativePtr.get dout 1), c2[2]) ^^^ ROTL64((NativePtr.get dout 2), c2[3]) ^^^ ROTL64((NativePtr.get dout 3), c2[4]) ^^^ ROTL64((NativePtr.get dout 4), c2[5]) ^^^ ROTL64((NativePtr.get dout 5), c2[6]))
    NativePtr.set dout 7 ((NativePtr.get din 7) ^^^ ROTL64((NativePtr.get dout 0), c2[0]) ^^^ ROTL64((NativePtr.get dout 1), c2[1]) ^^^ ROTL64((NativePtr.get dout 2), c2[2]) ^^^ ROTL64((NativePtr.get dout 3), c2[3]) ^^^ ROTL64((NativePtr.get dout 4), c2[4]) ^^^ ROTL64((NativePtr.get dout 5), c2[5]) ^^^ ROTL64((NativePtr.get dout 6), c2[6]))

let linOp (d: nativeptr<byte>, r: nativeptr<byte>, blocklen: int) =
    match blocklen with
    | 16 -> lin344(d |> NativePtr.toVoidPtr |> NativePtr.ofVoidPtr, r |> NativePtr.toVoidPtr |> NativePtr.ofVoidPtr)
    | 32 -> lin384(d |> NativePtr.toVoidPtr |> NativePtr.ofVoidPtr, r |> NativePtr.toVoidPtr |> NativePtr.ofVoidPtr)
    | 64 -> lin388(d |> NativePtr.toVoidPtr |> NativePtr.ofVoidPtr, r |> NativePtr.toVoidPtr |> NativePtr.ofVoidPtr)
    | _ -> assert false

let sBox (data: nativeptr<byte>, res: nativeptr<byte>,  blen: int)=
    for i = 0 to blen - 1 do
        NativePtr.set res i sb[int (NativePtr.get data i)]

let AddRkX(block: nativeptr<byte>, rkey: nativeptr<byte>, nr, blen, res: nativeptr<byte>)=
    for i = 0 to blen - 1 do
        NativePtr.set res i ((NativePtr.get block i) ^^^ (NativePtr.get rkey (nr * blen + i)))

let AddRk(block: nativeptr<byte>, rkey: nativeptr<byte>, nr, blen, res: nativeptr<byte>)=
    let mutable tmp = (uint16 (NativePtr.get block 0) + uint16 (NativePtr.get rkey (nr * blen)))
    NativePtr.set res 0 (byte tmp)
    tmp <- tmp >>> 8
    for i = 1 to blen - 1 do
        tmp <- tmp + (uint16 (NativePtr.get block i) + uint16 (NativePtr.get rkey (nr * blen + i)))
        NativePtr.set res i (byte tmp)
        tmp <- tmp >>> 8

let encrypt(data: nativeptr<byte>, rkey: nativeptr<byte>, klen, blen, res: nativeptr<byte>) =
    let block = NativePtr.stackalloc<byte> MAXBLOCKLEN
    let block2 = NativePtr.stackalloc<byte> MAXBLOCKLEN
    AddRk(data, rkey, 0, blen, block)
    sBox(block, block2, blen)
    linOp(block2, block, blen)
    for i = 1 to blen - 2 do
        AddRkX(block, rkey, i, blen, block2)
        sBox(block2, block2, blen)
        linOp(block2, block, blen)

    AddRk(block, rkey, RNDS(klen) - 1, blen, res)

let InvAddRk(block: nativeptr<byte>, rkey: nativeptr<byte>, nr, blen, res: nativeptr<byte>)=
    let mutable tmp = (uint16 (NativePtr.get block 0) - uint16 (NativePtr.get rkey (nr * blen)))
    NativePtr.set res 0 (byte tmp)
    tmp <- tmp >>> 8
    for i = 1 to blen - 1 do
        tmp <- tmp + (uint16 (NativePtr.get block i) - uint16 (NativePtr.get rkey (nr * blen + i)))
        NativePtr.set res i (byte tmp)
        tmp <- tmp >>> 8

let ilin344 (din: nativeptr<uint>, dout: nativeptr<uint>) =
    NativePtr.set dout 3 ((NativePtr.get din 3) ^^^ ROTL((NativePtr.get din 0), c0[0]) ^^^ ROTL((NativePtr.get din 1), c0[1]) ^^^ ROTL((NativePtr.get din 2), c0[2]))
    NativePtr.set dout 2 ((NativePtr.get din 2) ^^^ ROTL((NativePtr.get dout 3), c0[0]) ^^^ ROTL((NativePtr.get din 0), c0[1]) ^^^ ROTL((NativePtr.get din 1), c0[2]))
    NativePtr.set dout 1 ((NativePtr.get din 1) ^^^ ROTL((NativePtr.get dout 2), c0[0]) ^^^ ROTL((NativePtr.get dout 3), c0[1]) ^^^ ROTL((NativePtr.get din 0), c0[2]))
    NativePtr.set dout 0 ((NativePtr.get din 0) ^^^ ROTL((NativePtr.get dout 1), c0[0]) ^^^ ROTL((NativePtr.get dout 2), c0[1]) ^^^ ROTL((NativePtr.get dout 3), c0[2]))

let ilin384 (din: nativeptr<uint>, dout: nativeptr<uint>) =
    NativePtr.set dout 7 ((NativePtr.get din 7) ^^^ ROTL((NativePtr.get din 0), c1[0]) ^^^ ROTL((NativePtr.get din 1), c1[1]) ^^^ ROTL((NativePtr.get din 2), c1[2]) ^^^ ROTL((NativePtr.get din 3), c1[3]) ^^^ ROTL((NativePtr.get din 4), c1[4]) ^^^ ROTL((NativePtr.get din 5), c1[5]) ^^^ ROTL((NativePtr.get din 6), c1[6]))
    NativePtr.set dout 6 ((NativePtr.get din 6) ^^^ ROTL((NativePtr.get dout 7), c1[0]) ^^^ ROTL((NativePtr.get din 0), c1[1]) ^^^ ROTL((NativePtr.get din 1), c1[2]) ^^^ ROTL((NativePtr.get din 2), c1[3]) ^^^ ROTL((NativePtr.get din 3), c1[4]) ^^^ ROTL((NativePtr.get din 4), c1[5]) ^^^ ROTL((NativePtr.get din 5), c1[6]))
    NativePtr.set dout 5 ((NativePtr.get din 5) ^^^ ROTL((NativePtr.get dout 6), c1[0]) ^^^ ROTL((NativePtr.get dout 7), c1[1]) ^^^ ROTL((NativePtr.get din 0), c1[2]) ^^^ ROTL((NativePtr.get din 1), c1[3]) ^^^ ROTL((NativePtr.get din 2), c1[4]) ^^^ ROTL((NativePtr.get din 3), c1[5]) ^^^ ROTL((NativePtr.get din 4), c1[6]))
    NativePtr.set dout 4 ((NativePtr.get din 4) ^^^ ROTL((NativePtr.get dout 5), c1[0]) ^^^ ROTL((NativePtr.get dout 6), c1[1]) ^^^ ROTL((NativePtr.get dout 7), c1[2]) ^^^ ROTL((NativePtr.get din 0), c1[3]) ^^^ ROTL((NativePtr.get din 1), c1[4]) ^^^ ROTL((NativePtr.get din 2), c1[5]) ^^^ ROTL((NativePtr.get din 3), c1[6]))
    NativePtr.set dout 3 ((NativePtr.get din 3) ^^^ ROTL((NativePtr.get dout 4), c1[0]) ^^^ ROTL((NativePtr.get dout 5), c1[1]) ^^^ ROTL((NativePtr.get dout 6), c1[2]) ^^^ ROTL((NativePtr.get dout 7), c1[3]) ^^^ ROTL((NativePtr.get din 0), c1[4]) ^^^ ROTL((NativePtr.get din 1), c1[5]) ^^^ ROTL((NativePtr.get din 2), c1[6]))
    NativePtr.set dout 2 ((NativePtr.get din 2) ^^^ ROTL((NativePtr.get dout 3), c1[0]) ^^^ ROTL((NativePtr.get dout 4), c1[1]) ^^^ ROTL((NativePtr.get dout 5), c1[2]) ^^^ ROTL((NativePtr.get dout 6), c1[3]) ^^^ ROTL((NativePtr.get dout 7), c1[4]) ^^^ ROTL((NativePtr.get din 0), c1[5]) ^^^ ROTL((NativePtr.get din 1), c1[6]))
    NativePtr.set dout 1 ((NativePtr.get din 1) ^^^ ROTL((NativePtr.get dout 2), c1[0]) ^^^ ROTL((NativePtr.get dout 3), c1[1]) ^^^ ROTL((NativePtr.get dout 4), c1[2]) ^^^ ROTL((NativePtr.get dout 5), c1[3]) ^^^ ROTL((NativePtr.get dout 6), c1[4]) ^^^ ROTL((NativePtr.get dout 7), c1[5]) ^^^ ROTL((NativePtr.get din 0), c1[6]))
    NativePtr.set dout 0 ((NativePtr.get din 0) ^^^ ROTL((NativePtr.get dout 1), c1[0]) ^^^ ROTL((NativePtr.get dout 2), c1[1]) ^^^ ROTL((NativePtr.get dout 3), c1[2]) ^^^ ROTL((NativePtr.get dout 4), c1[3]) ^^^ ROTL((NativePtr.get dout 5), c1[4]) ^^^ ROTL((NativePtr.get dout 6), c1[5]) ^^^ ROTL((NativePtr.get dout 7), c1[6]))

let ilin388 (din: nativeptr<uint64>, dout: nativeptr<uint64>) =
    NativePtr.set dout 7 ((NativePtr.get din 7) ^^^ ROTL64((NativePtr.get din 0), c1[0]) ^^^ ROTL64((NativePtr.get din 1), c1[1]) ^^^ ROTL64((NativePtr.get din 2), c1[2]) ^^^ ROTL64((NativePtr.get din 3), c1[3]) ^^^ ROTL64((NativePtr.get din 4), c1[4]) ^^^ ROTL64((NativePtr.get din 5), c1[5]) ^^^ ROTL64((NativePtr.get din 6), c1[6]))
    NativePtr.set dout 6 ((NativePtr.get din 6) ^^^ ROTL64((NativePtr.get dout 7), c1[0]) ^^^ ROTL64((NativePtr.get din 0), c1[1]) ^^^ ROTL64((NativePtr.get din 1), c1[2]) ^^^ ROTL64((NativePtr.get din 2), c1[3]) ^^^ ROTL64((NativePtr.get din 3), c1[4]) ^^^ ROTL64((NativePtr.get din 4), c1[5]) ^^^ ROTL64((NativePtr.get din 5), c1[6]))
    NativePtr.set dout 5 ((NativePtr.get din 5) ^^^ ROTL64((NativePtr.get dout 6), c1[0]) ^^^ ROTL64((NativePtr.get dout 7), c1[1]) ^^^ ROTL64((NativePtr.get din 0), c1[2]) ^^^ ROTL64((NativePtr.get din 1), c1[3]) ^^^ ROTL64((NativePtr.get din 2), c1[4]) ^^^ ROTL64((NativePtr.get din 3), c1[5]) ^^^ ROTL64((NativePtr.get din 4), c1[6]))
    NativePtr.set dout 4 ((NativePtr.get din 4) ^^^ ROTL64((NativePtr.get dout 5), c1[0]) ^^^ ROTL64((NativePtr.get dout 6), c1[1]) ^^^ ROTL64((NativePtr.get dout 7), c1[2]) ^^^ ROTL64((NativePtr.get din 0), c1[3]) ^^^ ROTL64((NativePtr.get din 1), c1[4]) ^^^ ROTL64((NativePtr.get din 2), c1[5]) ^^^ ROTL64((NativePtr.get din 3), c1[6]))
    NativePtr.set dout 3 ((NativePtr.get din 3) ^^^ ROTL64((NativePtr.get dout 4), c1[0]) ^^^ ROTL64((NativePtr.get dout 5), c1[1]) ^^^ ROTL64((NativePtr.get dout 6), c1[2]) ^^^ ROTL64((NativePtr.get dout 7), c1[3]) ^^^ ROTL64((NativePtr.get din 0), c1[4]) ^^^ ROTL64((NativePtr.get din 1), c1[5]) ^^^ ROTL64((NativePtr.get din 2), c1[6]))
    NativePtr.set dout 2 ((NativePtr.get din 2) ^^^ ROTL64((NativePtr.get dout 3), c1[0]) ^^^ ROTL64((NativePtr.get dout 4), c1[1]) ^^^ ROTL64((NativePtr.get dout 5), c1[2]) ^^^ ROTL64((NativePtr.get dout 6), c1[3]) ^^^ ROTL64((NativePtr.get dout 7), c1[4]) ^^^ ROTL64((NativePtr.get din 0), c1[5]) ^^^ ROTL64((NativePtr.get din 1), c1[6]))
    NativePtr.set dout 1 ((NativePtr.get din 1) ^^^ ROTL64((NativePtr.get dout 2), c1[0]) ^^^ ROTL64((NativePtr.get dout 3), c1[1]) ^^^ ROTL64((NativePtr.get dout 4), c1[2]) ^^^ ROTL64((NativePtr.get dout 5), c1[3]) ^^^ ROTL64((NativePtr.get dout 6), c1[4]) ^^^ ROTL64((NativePtr.get dout 7), c1[5]) ^^^ ROTL64((NativePtr.get din 0), c1[6]))
    NativePtr.set dout 0 ((NativePtr.get din 0) ^^^ ROTL64((NativePtr.get dout 1), c1[0]) ^^^ ROTL64((NativePtr.get dout 2), c1[1]) ^^^ ROTL64((NativePtr.get dout 3), c1[2]) ^^^ ROTL64((NativePtr.get dout 4), c1[3]) ^^^ ROTL64((NativePtr.get dout 5), c1[4]) ^^^ ROTL64((NativePtr.get dout 6), c1[5]) ^^^ ROTL64((NativePtr.get dout 7), c1[6]))

let InvlinOp (d: nativeptr<byte>, r: nativeptr<byte>, blocklen: int) =
    match blocklen with
    | 16 -> ilin344(d |> NativePtr.toVoidPtr |> NativePtr.ofVoidPtr, r |> NativePtr.toVoidPtr |> NativePtr.ofVoidPtr)
    | 32 -> ilin384(d |> NativePtr.toVoidPtr |> NativePtr.ofVoidPtr, r |> NativePtr.toVoidPtr |> NativePtr.ofVoidPtr)
    | 64 -> ilin388(d |> NativePtr.toVoidPtr |> NativePtr.ofVoidPtr, r |> NativePtr.toVoidPtr |> NativePtr.ofVoidPtr)
    | _ -> assert false

let InvsBox (data: nativeptr<byte>, res: nativeptr<byte>,  blen: int)=
    for i = 0 to blen - 1 do
        NativePtr.set res i isb[int (NativePtr.get data i)]

let decrypt(data: nativeptr<byte>, rkey: nativeptr<byte>, klen, blen, res: nativeptr<byte>) =
    let block = NativePtr.stackalloc<byte> MAXBLOCKLEN
    let block2 = NativePtr.stackalloc<byte> MAXBLOCKLEN
    InvAddRk(data, rkey, RNDS(klen) - 1, blen, block)
    for i = blen - 2 downto 1 do
        InvlinOp(block, block2, blen)
        InvsBox(block2, block2, blen)
        AddRkX(block2, rkey, i, blen, block)
        
    InvlinOp(block, block2, blen)
    InvsBox(block2, block2, blen)
    InvAddRk(block2, rkey, 0, blen, res)
