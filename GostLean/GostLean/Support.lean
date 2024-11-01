

class Serializable (α : Sort u) extends Inhabited α where
  sizeInBytes : Nat
  --sizeNotZero: LT.lt 0 sizeInBytes
  getFromBytes : (a: Array UInt8) -> Fin (a.size / sizeInBytes) → α
  setFromBytes : (a: Array UInt8) -> Fin (a.size / sizeInBytes) → α → Array UInt8
  --size_set

-- @[simp] theorem size_set (a : Array UInt8) (i : Fin (a.size / sizeInBytes)) (v : α) : (Serializable.setFromBytes a i v).size = a.size :=
--   List.length_set ..

class InhabitedSerializable (α : Sort u) extends Serializable α, Inhabited α

export Serializable (getFromBytes setFromBytes sizeInBytes)

-- This instance is needed because `id` is not reducible
instance {α} [Serializable α] : Serializable (id α) :=
  inferInstanceAs (Serializable α)

instance {α} [Serializable α] : Serializable (Id α) :=
  inferInstanceAs (Serializable α)

instance : Serializable UInt8 where
  sizeInBytes := 1
  --sizeNotZero := (LT.lt 0 sizeInBytes)
  getFromBytes := fun a ndx => a[ndx]
  setFromBytes :=
    fun a ndx v =>
      have h: a.size / 1 ≤ a.size := by
        apply Nat.div_le_self
      a.set (Fin.castLE h ndx) v

namespace Nat

theorem lt_div_iff_mul_le (k0 : 0 < k) (k1 : t < k) : x < y / k ↔ x * k + t < y := by
  -- induction y, k using Nat.mod.inductionOn generalizing x with
  --   (rw [div_eq]; simp [h]; cases x with | zero => simp [zero_le] | succ x => ?_)
  -- | base y k h =>
  --   simp only [add_one, succ_mul, false_iff, Nat.not_le]
  --   refine Nat.lt_of_lt_of_le ?_ (Nat.le_add_left ..)
  --   exact Nat.not_le.1 fun h' => h ⟨k0, h'⟩
  -- | ind y k h IH =>
  --   rw [Nat.add_le_add_iff_right, IH k0, succ_mul,
  --       ← Nat.add_sub_cancel (x*k) k, Nat.sub_le_sub_iff_right h.2, Nat.add_sub_cancel]
  admit
end Nat

namespace Fin
def div_mul : Fin n → (m: Nat) → (a0 : 0 < m) → Fin (n * m)
  | ⟨a, h⟩, m, t =>
    have :a*m < n*m := by
      simp_all
    ⟨a * m, this⟩
def mul_div : (m: Nat) → Fin (n / m) → (a0 : 0 < m) → Fin (n)
  | m, ⟨a, h⟩, t =>
    have zz: n / m * m ≤ n % m + n / m * m := by
      simp_all
    have : a * m < n := by
      rw [← Nat.mul_lt_mul_right t] at h
      -- rw [← Nat.div_mul_cancel] at h
      -- rw [Nat.lt_add_left n]
      -- rw [Nat.le_add_left (n / m * m) (n % m)] at h
      rw [← Nat.mod_add_div n m]
      rw [Nat.mul_comm m]
      --rw [← Nat.mul_lt_mul_right t]
      --simp [Nat.lt_of_lt_of_le,Nat.le_add_left] at h
      apply Nat.lt_of_lt_of_le h zz
    ⟨a*m, this⟩

theorem my_add_lt_add_of_le_of_lt {a b c d e : Nat} (hlt : d < e) (hle : a + b < c + d) :
    a + b < c + e :=
  Nat.lt_of_le_of_lt (Nat.le_of_lt hle) (Nat.add_lt_add_left hlt c)

def mul_div_add : (m k: Nat) → Fin (n / m) → (a0 : 0 < m) → (a1 : k < m) → Fin (n)
  | m, k, ⟨a, h⟩, t, t2 =>
    have zzz : n / m * m + n % m = n := by
      rw [Nat.mul_comm, Nat.div_add_mod n m]
    have zz4: a * m < (n / m) * m := by
      simp [(Nat.mul_lt_mul_right t)]
      exact h
    have : a*m + k < n := by
      rw [← zzz]
      cases m
      apply absurd t2 (Nat.not_lt_zero _)
      sorry
      -- rw [Nat.le_div_iff_mul_le]
      -- apply Nat.lt_add_one_iff_lt_or_eq
      --apply (Nat.mod_lt m) a0
      --apply Nat.lt_of_add_lt_add_left zz4 t2
      --apply ((Nat.mod_lt m) t)
      -- apply my_add_lt_add_of_le_of_lt zz4 ((Nat.mod_lt m) t)

    ⟨a*m + k, this⟩
end Fin

instance : Serializable UInt32 where
  sizeInBytes := 4
  --sizeNotZero := (LT.lt 0 sizeInBytes)
  getFromBytes :=
    fun a ndx =>
      have h1: 0 < 4 := by
        trivial
      have h2: 0 < 4 := by
        simp_all
      have h3: 1 < 4 := by
        simp_all
      have h4: 2 < 4 := by
        simp_all
      have h5: 3 < 4 := by
        simp_all

      let baseNdx := Fin.mul_div_add 4 0 ndx h1 h2
      let base1Ndx := Fin.mul_div_add 4 1 ndx h1 h3
      let base2Ndx := Fin.mul_div_add 4 2 ndx h1 h4
      let base3Ndx := Fin.mul_div_add 4 3 ndx h1 h5
      UInt32.ofNat ((UInt8.toNat a[baseNdx] <<< 24)
        + (UInt8.toNat a[base1Ndx] <<< 16)
        + (UInt8.toNat a[base2Ndx] <<< 8)
        + (UInt8.toNat a[base3Ndx] <<< 0))
  setFromBytes :=
    fun a ndx v =>
      have h1: 0 < 4 := by
        trivial
      have h2: 0 < 4 := by
        simp_all
      have h3: 1 < 4 := by
        simp_all
      have h4: 2 < 4 := by
        simp_all
      have h5: 3 < 4 := by
        simp_all

      let baseNdx := Fin.mul_div_add 4 0 ndx h1 h2
      let base1Ndx := Fin.mul_div_add 4 1 ndx h1 h3
      let base2Ndx := Fin.mul_div_add 4 2 ndx h1 h4
      let base3Ndx := Fin.mul_div_add 4 3 ndx h1 h5
      let byte1 := UInt32.toUInt8 (v >>> 24)
      let byte2 := UInt32.toUInt8 (v >>> 16)
      let byte3 := UInt32.toUInt8 (v >>> 8)
      let byte4 := UInt32.toUInt8 v
      let a1 := a.set baseNdx byte1
      let a2 := a1.set ⟨base1Ndx, by simp [a.size_set]⟩ byte2
      let a3 := a2.set ⟨base2Ndx, by simp [a.size_set, a1.size_set]⟩ byte3
      let a4 := a3.set ⟨base3Ndx, by simp [a.size_set, a1.size_set, a2.size_set]⟩ byte4
      a4
-- instance [Inhabited UInt32] : InhabitedSerializable UInt32 where
--   default := ‹Inhabited R›

instance : Serializable UInt64 where
  sizeInBytes := 8
  --sizeNotZero := (LT.lt 0 sizeInBytes)
  getFromBytes :=
    fun a ndx =>
      have h0: 0 < 8 := by
        simp_all
      have h1: 1 < 8 := by
        simp_all
      have h2: 2 < 8 := by
        simp_all
      have h3: 3 < 8 := by
        simp_all
      have h4: 4 < 8 := by
        simp_all
      have h5: 5 < 8 := by
        simp_all
      have h6: 6 < 8 := by
        simp_all
      have h7: 7 < 8 := by
        simp_all

      let base0Ndx := Fin.mul_div_add 8 0 ndx h0 h0
      let base1Ndx := Fin.mul_div_add 8 1 ndx h0 h1
      let base2Ndx := Fin.mul_div_add 8 2 ndx h0 h2
      let base3Ndx := Fin.mul_div_add 8 3 ndx h0 h3
      let base4Ndx := Fin.mul_div_add 8 4 ndx h0 h4
      let base5Ndx := Fin.mul_div_add 8 5 ndx h0 h5
      let base6Ndx := Fin.mul_div_add 8 6 ndx h0 h6
      let base7Ndx := Fin.mul_div_add 8 7 ndx h0 h7
      UInt64.ofNat ((UInt8.toNat a[base0Ndx] <<< 56)
        + (UInt8.toNat a[base1Ndx] <<< 48)
        + (UInt8.toNat a[base2Ndx] <<< 40)
        + (UInt8.toNat a[base3Ndx] <<< 32)
        + (UInt8.toNat a[base4Ndx] <<< 24)
        + (UInt8.toNat a[base5Ndx] <<< 16)
        + (UInt8.toNat a[base6Ndx] <<< 8)
        + (UInt8.toNat a[base7Ndx] <<< 0))
  setFromBytes :=
    fun a ndx v =>
      have h0: 0 < 8 := by
        simp_all
      have h1: 1 < 8 := by
        simp_all
      have h2: 2 < 8 := by
        simp_all
      have h3: 3 < 8 := by
        simp_all
      have h4: 4 < 8 := by
        simp_all
      have h5: 5 < 8 := by
        simp_all
      have h6: 6 < 8 := by
        simp_all
      have h7: 7 < 8 := by
        simp_all

      let base0Ndx := Fin.mul_div_add 8 0 ndx h0 h0
      let base1Ndx := Fin.mul_div_add 8 1 ndx h0 h1
      let base2Ndx := Fin.mul_div_add 8 2 ndx h0 h2
      let base3Ndx := Fin.mul_div_add 8 3 ndx h0 h3
      let base4Ndx := Fin.mul_div_add 8 4 ndx h0 h4
      let base5Ndx := Fin.mul_div_add 8 5 ndx h0 h5
      let base6Ndx := Fin.mul_div_add 8 6 ndx h0 h6
      let base7Ndx := Fin.mul_div_add 8 7 ndx h0 h7
      let byte0 := UInt64.toUInt8 (v >>> 56)
      let byte1 := UInt64.toUInt8 (v >>> 48)
      let byte2 := UInt64.toUInt8 (v >>> 40)
      let byte3 := UInt64.toUInt8 (v >>> 32)
      let byte4 := UInt64.toUInt8 (v >>> 24)
      let byte5 := UInt64.toUInt8 (v >>> 16)
      let byte6 := UInt64.toUInt8 (v >>> 8)
      let byte7 := UInt64.toUInt8 v
      let a1 := a.set base0Ndx byte0
      let a2 := a1.set ⟨base1Ndx, by simp [a.size_set]⟩ byte1
      let a3 := a2.set ⟨base2Ndx, by simp [a.size_set, a1.size_set]⟩ byte2
      let a4 := a3.set ⟨base3Ndx, by simp [a.size_set, a1.size_set, a2.size_set]⟩ byte3
      let a5 := a4.set ⟨base4Ndx, by simp [a.size_set, a1.size_set, a2.size_set, a3.size_set]⟩ byte4
      let a6 := a5.set ⟨base5Ndx, by simp [a.size_set, a1.size_set, a2.size_set, a3.size_set, a4.size_set]⟩ byte5
      let a7 := a6.set ⟨base6Ndx, by simp [a.size_set, a1.size_set, a2.size_set, a3.size_set, a4.size_set, a5.size_set]⟩ byte6
      let a8 := a7.set ⟨base7Ndx, by simp [a.size_set, a1.size_set, a2.size_set, a3.size_set, a4.size_set, a5.size_set, a6.size_set]⟩ byte7
      a8

structure Subarray2 (α : Type u) [Serializable α] where
  array : Array UInt8
  start : Nat
  stop : Nat
  start_le_stop : start ≤ stop
  stop_le_array_size : stop ≤ array.size / sizeInBytes α
  item_size_ge_zero : 0 < sizeInBytes α

namespace Subarray2

def size (α : Type u) [Serializable α] (s : Subarray2 α) : Nat :=
  s.stop - s.start

-- theorem size_le_array_size [Serializable α] {s : Subarray2 α} : s.size * (sizeInBytes α) ≤ s.array.size := by
--   let {array, start, stop, start_le_stop, stop_le_array_size, item_size_ge_zero} := s
--   simp [size]
--   apply Nat.le_trans (Nat.sub_le stop start)
--   assumption

def get (α : Type u) [Serializable α] (s : Subarray2 α) (i : Fin s.size) : α :=
  have : (s.start + i.val) < s.array.size / sizeInBytes α := by
   --apply Nat.le_of_lt
   apply Nat.lt_of_lt_of_le _ s.stop_le_array_size
   have := i.isLt
   simp only [size] at this
   --rw [Fin.val] at this
   --apply Fin.mk_val at i
   --rw [Nat.mul_lt_mul_right s.item_size_ge_zero]
   rw [Nat.add_comm]
   exact Nat.add_lt_of_lt_sub this
  --let x : Subarray := 1
  let z : Fin (s.array.size / sizeInBytes α) := ⟨s.start + i.val, this⟩
  Serializable.getFromBytes s.array z

-- set_option diagnostics true
-- instance : GetElem (α : Type u) (Subarray2 α) Nat α [Serializable α] fun xs i => i < xs.size where
--   getElem xs i h := xs.get ⟨i, h⟩

@[inline] def getD (α : Type u) [Serializable α] (s : Subarray2 α) (i : Nat) (v₀ : α) : α :=
  if h : i < s.size then s.get α ⟨i, h⟩ else v₀

abbrev get! (α : Type u) [Serializable α] (s : Subarray2 α) (i : Nat) : α :=
  getD α s i default

def set (α : Type u) [Serializable α] (s : Subarray2 α) (i : Fin s.size) (v: α): Subarray2 α :=
  have : s.start + i.val < s.array.size / sizeInBytes α := by
   apply Nat.lt_of_lt_of_le _ s.stop_le_array_size
   have := i.isLt
   simp only [size] at this
   rw [Nat.add_comm]
   exact Nat.add_lt_of_lt_sub this
  let z : Fin (s.array.size / sizeInBytes α) := ⟨s.start + i.val, this⟩
  let changed_value := Serializable.setFromBytes s.array z v
  have : s.stop ≤ (changed_value.size / sizeInBytes α) := by
   --simp [s.array.size_set]
   sorry
  if h : s.stop ≤ changed_value.size / sizeInBytes α then {
        array := changed_value,
        stop := s.stop,
        start:= s.start,
        start_le_stop:= s.start_le_stop,
        stop_le_array_size := h
        item_size_ge_zero:= s.item_size_ge_zero,
    } else s

@[inline] def setD [Serializable α] (s : Subarray2 α) (i : Nat) (v₀ : α) : Subarray2 α :=
  if h : i < s.size then s.set α ⟨i, h⟩ v₀ else s

abbrev set! [Inhabited α] [Serializable α] (s : Subarray2 α) (i : Nat) (v₀ : α) : Subarray2 α :=
  setD s i v₀

end Subarray2
