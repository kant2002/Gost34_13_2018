import GostLean.Qalqan
import GostLean.PRNG

def random_arr (size:Nat) : IO ByteArray := do
  let mut key := ByteArray.mk (mkArray (α := UInt8) MAXKEYLEN 0)
  for i in [0:MAXKEYLEN - 1] do
    let value := ← rnext3
    key := key.set! i value
  pure key

def ShortTestKExp (f: IO.FS.Stream) (src: Option IO.FS.Stream) : (IO Unit) := do
    -- let mut key := mkArray (α := UInt8) MAXKEYLEN 0
    -- let rkey: = NativePtr.stackalloc<byte> (RNDS(MAXKEYLEN) * MAXBLOCKLEN)
    let key: IO ByteArray :=
      match src with
      | some src  => src.read (USize.ofNat MAXKEYLEN)
      | none      => random_arr MAXKEYLEN

    f.putStr s!"\nKey expansion for {(MAXKEYLEN * 8)} bit key and {MAXBLOCKLEN * 8} bit block:\n"
    -- KexpVV(key, MAXKEYLEN, MAXBLOCKLEN, rkey, f)
    f.putStr "\n"

def ShortTestVectors (f: IO.FS.Stream) (src: Option IO.FS.Stream) : (IO Unit) := do

    -- ShortTestLin(f, src);
    -- ShortTestLin(f, src);
    -- ShortTestSBox(f, src);
    ShortTestKExp f src
    -- //ShortTestBasicEnc(f);
    -- ShortTestEnc(f, src);
