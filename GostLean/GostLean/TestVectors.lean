import GostLean.Qalqan
import GostLean.PRNG

def toHex (n : UInt8) : String :=
    let d2 := n / 16;
    let d1 := n % 16;
    hexDigitRepr (UInt8.toNat d2) ++ hexDigitRepr (UInt8.toNat d1)

def random_arr (size:Nat) : IO ByteArray := do
  let mut key := ByteArray.mk (mkArray (α := UInt8) MAXKEYLEN 0)
  for i in [0:MAXKEYLEN - 1] do
    let value := ← rnext3
    key := key.set! i value
  pure key

def Pr3(b: Array UInt8) (len: Nat) (f: IO.FS.Stream) := do
    for i in  [0:len - 1] do
        f.putStr s!"{toHex b[i]!}" -- :x2
    f.putStr "\n"

def Pr(str: String) (b: Array UInt8) (len: Nat) (f: IO.FS.Stream) := do
    f.putStr s!"{str}: \n"
    for i in [0:len - 1] do
        f.putStr s!"{toHex b[i]!}" -- :x2
    f.putStr "\n"

def KexpVV(key: Array UInt8) (blen: Nat) (rkey: Array UInt8) (f: IO.FS.Stream) : IO (Array UInt8) := do
    Pr "Initial key" key key.size f
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
    Pr "Register L0" r0 17 f
    Pr "Register L1" r1 15 f
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
                        if (step < 32) then
                            f.putStr s!"Additional key byte to L0: {toHex key[(32 + step)]!}\n"
                        t0 := t0 + key[32 + step]!
                    else
                        if (step < 32) then
                            f.putStr s!"Additional key byte to L1: {toHex key[(32 + step)]!}\n"
                        t1 := t1 + key[32 + step]!
                    if (step < 32) then
                      Pr "Register L0" r0 17 f
                      Pr "Register L1" r1 15 f
                    step := step + 1

                if (step < 8) then
                    f.putStr s!"Output {toHex (t0 + r1[4]! &&& 0xff)} (L0: {toHex t0}, L1: {toHex r1[4]!})\n"
            r0 := Array.set! r0 16 t0
            r1 := Array.set! r1 14 t1
            if (step < 16) then
                f.putStr s!"Step {k}\nFeedback L0: {toHex t0}, L1: {toHex t1}\n"
                Pr "Register L0" r0 17 f
                Pr "Register L1" r1 15 f
        s := 0
    return rkey_local

def ShortTestKExp (f: IO.FS.Stream) (src: Option IO.FS.Stream) : (IO Unit) := do
    -- let mut key := mkArray (α := UInt8) MAXKEYLEN 0
    let rkey := mkArray (α := UInt8) ((RNDS MAXKEYLEN) * MAXBLOCKLEN) 0
    let key: IO ByteArray :=
      match src with
      | some src  => src.read (USize.ofNat MAXKEYLEN)
      | none      => random_arr MAXKEYLEN

    f.putStr s!"\nKey expansion for {(MAXKEYLEN * 8)} bit key and {MAXBLOCKLEN * 8} bit block:\n"
    let key := (← key).data
    let dummy ← KexpVV key MAXBLOCKLEN rkey f
    f.putStr "\n"

def ShortTestVectors (f: IO.FS.Stream) (src: Option IO.FS.Stream) : (IO Unit) := do

    -- ShortTestLin(f, src);
    -- ShortTestLin(f, src);
    -- ShortTestSBox(f, src);
    ShortTestKExp f src
    -- //ShortTestBasicEnc(f);
    -- ShortTestEnc(f, src);
