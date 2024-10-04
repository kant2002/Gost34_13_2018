-- def x := 1

-- def init (new_x: int) :=
--     x <- new_x

-- def rnext () :=
--     x <- (1103515245 * x + 12345)
--     byte (x >>> 16)

structure rng_state where
  x : UInt32
deriving Repr

def init : UInt32 -> rng_state :=
    fun (state) => { x := state }

def rnext (state: rng_state) : UInt8 × rng_state :=
    let new_x := (1103515245 * state.x + 12345)
    let sh: UInt8 := UInt32.toUInt8 (new_x >>> 16)
    (sh, { x := new_x })
