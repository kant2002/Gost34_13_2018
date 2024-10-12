import GostLean

def main : IO Unit := do
  let state := init2 0
  let src ← IO.FS.Handle.mk "../resources/input.bin" IO.FS.Mode.read
  let f ← IO.FS.Handle.mk "../resources/output_lean.txt" IO.FS.Mode.write
  let f := IO.FS.Stream.ofHandle f
  let src := IO.FS.Stream.ofHandle src
  ShortTestVectors f (some src)
  f.flush
