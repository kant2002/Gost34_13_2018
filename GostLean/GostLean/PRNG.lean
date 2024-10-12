-- def x := 1

-- def init (new_x: int) :=
--     x <- new_x

-- def rnext () :=
--     x <- (1103515245 * x + 12345)
--     byte (x >>> 16)

structure rng_state where
  x : UInt32
deriving Repr

initialize prngRef : IO.Ref UInt32 ←
  IO.mkRef (0)

def init3 (n : UInt32) : IO Unit :=
  prngRef.set n

def rnext3 : IO UInt8 := do
  let gen ← prngRef.get
  let new_x := (1103515245 * gen + 12345)
  let sh: UInt8 := UInt32.toUInt8 (new_x >>> 16)
  prngRef.set new_x
  pure sh

def init2 (state: UInt32) : StateM rng_state Unit := do
  let new_state : rng_state := { x := state }
  set new_state

def rnext (state: rng_state) : UInt8 × rng_state :=
    let new_x := (1103515245 * state.x + 12345)
    let sh: UInt8 := UInt32.toUInt8 (new_x >>> 16)
    (sh, { x := new_x })

def init : UInt32 -> rng_state :=
    fun (state) => { x := state }

def rnext2 : StateM rng_state UInt8 := do
    let state := ← get
    let new_x := (1103515245 * state.x + 12345)
    let new_state : rng_state := { x := new_x }
    set new_state
    let sh: UInt8 := UInt32.toUInt8 (new_x >>> 16)
    return sh
