class Serializable (α : Sort u) where
  getFromBytes : (a: Array UInt8) -> Fin (a.size / (sizeOf α)) → α
  setFromBytes : (a: Array UInt8) -> Fin (a.size / (sizeOf α)) → α → Array UInt8

export Serializable (getFromBytes setFromBytes)

-- This instance is needed because `id` is not reducible
instance {α} [Serializable α] : Serializable (id α) :=
  inferInstanceAs (Serializable α)

instance {α} [Serializable α] : Serializable (Id α) :=
  inferInstanceAs (Serializable α)

instance : Serializable UInt8 where
  getFromBytes := fun a ndx => a[ndx]
  setFromBytes :=
    fun a ndx v =>
      have h: a.size / (sizeOf UInt8) ≤ a.size := by
        apply Nat.div_le_self
      a.set (Fin.castLE h ndx) v

structure Subarray2 (α : Type u) [SizeOf α] [Serializable α] where
  array : Array UInt8
  start : Nat
  stop : Nat
  start_le_stop : start ≤ stop
  stop_le_array_size : stop * (sizeOf α)  ≤ array.size

namespace Subarray2

def size (α : Type u) [SizeOf α] [Serializable α] (s : Subarray2 α) : Nat :=
  s.stop - s.start

-- theorem size_le_array_size {s : Subarray2 α} : s.size * (sizeOf α) ≤ s.array.size := by
--   let {array, start, stop, start_le_stop, stop_le_array_size} := s
--   simp [size]
--   apply Nat.le_trans (Nat.sub_le stop start)
--   assumption

-- def get (α : Type u) [SizeOf α] [Serializable α] (s : Subarray2 α) (i : Fin (s.size / (sizeOf α))) : α :=
--   have : (s.start + i.val) * (sizeOf α) < s.array.size := by
--    apply Nat.lt_of_lt_of_le _ s.stop_le_array_size
--    have := i.isLt
--    simp only [size] at this
--    rw [Nat.add_comm]
--    sorry
--   --  rw [Nat.add_lt_of_lt_sub ] at this
--   --  rw [Nat.le_of_mul_le_mul_left] at this
--   --  apply [← Nat.le_of_mul_le_mul_left]
--   --  exact Nat.add_lt_of_lt_sub this
--   Serializable.getFromBytes s.array (s.start + i.val)

-- instance : GetElem (α : Type u) [SizeOf α] [Serializable α] (Subarray2 α) Nat α fun xs i => i < xs.size where
--   getElem xs i h := xs.get ⟨i, h⟩

-- @[inline] def getD (α : Type u) [SizeOf α] [Serializable α] (s : Subarray2 α) (i : Nat) (v₀ : α) : α :=
--   if h : i < s.size then s.get ⟨i, h⟩ else v₀

-- abbrev get! [Inhabited α] (α : Type u) [SizeOf α] [Serializable α] (s : Subarray2 α) (i : Nat) : α :=
--   getD s i default

-- def set (α : Type u) [SizeOf α] [Serializable α] (s : Subarray2 α) (i : Fin s.size) (v: α): Subarray2 α :=
--   have : s.start + i.val < s.array.size := by
--    apply Nat.lt_of_lt_of_le _ s.stop_le_array_size
--    have := i.isLt
--    simp only [size] at this
--    rw [Nat.add_comm]
--    exact Nat.add_lt_of_lt_sub this
--   let changed_value := s.array.set ⟨s.start + i.val,this⟩ v
--   --have : s.stop ≤ changed_value.size := by
--   --  sorry
--   if h : s.stop ≤ changed_value.size then {
--         array := changed_value,
--         stop := s.stop,
--         start:= s.start,
--         start_le_stop:= s.start_le_stop,
--         stop_le_array_size := h
--     } else s

-- @[inline] def setD (s : Subarray2 α) (i : Nat) (v₀ : α) : Subarray2 α :=
--   if h : i < s.size then s.set ⟨i, h⟩ v₀ else s

-- abbrev set! [Inhabited α] (s : Subarray2 α) (i : Nat) (v₀ : α) : Subarray2 α :=
--   setD s i v₀

end Subarray2
