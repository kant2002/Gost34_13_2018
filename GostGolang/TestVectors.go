package main

import (
	"fmt"
	"io"
)

func Pr3(b []uint8, len int, f io.Writer) {
	for i := 0; i < len; i++ {
		fmt.Fprintf(f, "%02x", b[i])
	}

	fmt.Fprintf(f, "\n")
}

func Pr(str string, b []uint8, len int, f io.Writer) {
	fmt.Fprintf(f, "%s:\n", str)
	for i := 0; i < len; i++ {
		fmt.Fprintf(f, "%02x", b[i])
	}

	fmt.Fprintf(f, "\n")
}

func KexpV(key []uint8, klen int, blen int, rkey []uint8, f io.Writer) {
	Pr("Initial key", key, klen, f)
	var r0 = make([]uint8, 17)
	var r1 = make([]uint8, 15)
	var addk = klen - 32
	var step = 0
	var s = SHIFT
	for i := 0; i < 15; i++ {
		r0[i] = key[2*i]
		r1[i] = key[2*i+1]
	}
	r0[15] = key[30]
	r0[16] = key[31]
	Pr("Register L0", r0, 17, f)
	Pr("Register L1", r1, 15, f)
	for r := 0; r < RNDS(klen); r++ {
		for k := 0; k < blen+s; k++ {
			var t0 = (byte)(sb[r0[0]] + r0[1] + sb[r0[3]] + r0[7] + sb[r0[12]] + r0[16])
			var t1 = (byte)(sb[r1[0]] + r1[3] + sb[r1[9]] + r1[12] + sb[r1[14]])
			for i := 0; i < 14; i++ {
				r0[i] = r0[i+1]
				r1[i] = r1[i+1]
			}
			r0[14] = r0[15]
			r0[15] = r0[16]
			if k >= s {
				rkey[r*blen+k-s] = (byte)(t0 + r1[4])
				if step < addk {
					if (step & 1) != 0 {
						t0 += key[32+step]

					} else {
						t1 += key[32+step]
					}
					step++
				}
			}
			r0[16] = t0
			r1[14] = t1
		}
		if s != 0 {
			fmt.Fprintf(f, "After prerun:\n")
			Pr("Register L0", r0, 17, f)
			Pr("Register L1", r1, 15, f)
		}
		s = 0
	}
	for r := 0; r < RNDS(klen); r++ {
		fmt.Fprintf(f, "Round %2d key: ", r)
		Pr3(rkey[blen*r:], blen, f)
	}
}

func KexpVV(key []uint8, klen int, blen int, rkey []uint8, f io.Writer) {
	Pr("Initial key", key, klen, f)
	var r0 = make([]uint8, 17)
	var r1 = make([]uint8, 15)
	var addk = klen - 32
	var step = 0
	var s = SHIFT
	for i := 0; i < 15; i++ {
		r0[i] = key[2*i]
		r1[i] = key[2*i+1]
	}
	r0[15] = key[30]
	r0[16] = key[31]
	Pr("Register L0", r0, 17, f)
	Pr("Register L1", r1, 15, f)
	for r := 0; r < RNDS(klen); r++ {
		for k := 0; k < blen+s; k++ {
			var t0 = (byte)(sb[r0[0]] + r0[1] + sb[r0[3]] + r0[7] + sb[r0[12]] + r0[16])
			var t1 = (byte)(sb[r1[0]] + r1[3] + sb[r1[9]] + r1[12] + sb[r1[14]])
			for i := 0; i < 14; i++ {
				r0[i] = r0[i+1]
				r1[i] = r1[i+1]
			}
			r0[14] = r0[15]
			r0[15] = r0[16]
			if k >= s {
				rkey[r*blen+k-s] = (byte)(t0 + r1[4])
				if step < addk {
					if (step & 1) != 0 {
						if step < 32 {
							fmt.Fprintf(f, "Additional key byte to L0: %02x\n", key[32+step])
						}
						t0 += key[32+step]

					} else {
						if step < 32 {
							fmt.Fprintf(f, "Additional key byte to L1: %02x\n", key[32+step])
						}
						t1 += key[32+step]
					}
					if step < 32 {
						Pr("Register L0", r0, 17, f)
						Pr("Register L1", r1, 15, f)
					}
					step++
				}
				if step < 8 {
					fmt.Fprintf(f, "Output %02x (L0: %02x, L1: %02x)\n", (t0+r1[4])&0xff, t0, r1[4])
				}
			}
			r0[16] = t0
			r1[14] = t1
			//if (step < addk || (!r && (k < s)))
			if step < 16 {
				fmt.Fprintf(f, "Step %d\nFeedback L0: %02x, L1: %02x\n", k, t0, t1)
				Pr("Register L0", r0, 17, f)
				Pr("Register L1", r1, 15, f)
			}
		}
		s = 0
	}
	for r := 0; r < RNDS(klen); r++ {
		fmt.Fprintf(f, "Round %2d key: ", r)
		Pr3(rkey[blen*r:], blen, f)
	}
}

func encryptV(data []uint8, rkey []uint8, klen int, blen int, res []uint8, f io.Writer) {
	var block = make([]uint8, MAXBLOCKLEN)
	var block2 = make([]uint8, MAXBLOCKLEN)
	Pr("Clear text", data, blen, f)
	Pr("Key 0", rkey, blen, f)
	AddRk(data, rkey, 0, blen, block)
	Pr("After add K0", block, blen, f)
	sBox(block, block2, blen)
	Pr("After Sub", block2, blen, f)
	linOp(block2, block, blen)
	Pr("After linear", block, blen, f)
	for i := 1; i < RNDS(klen)-1; i++ {
		fmt.Fprintf(f, "Round %d.\nKey%d:\n", i, i)
		Pr3(rkey[blen*i:], blen, f)
		AddRkX(block, rkey, i, blen, block2)
		Pr("After addkey", block2, blen, f)
		sBox(block2, block2, blen)
		Pr("After Sub", block2, blen, f)
		linOp(block2, block, blen)
		Pr("After linear", block, blen, f)
	}
	Pr("Final key", rkey[blen*(RNDS(klen)-1):], blen, f)
	AddRk(block, rkey, RNDS(klen)-1, blen, res)
	Pr("Ciphertext", res, blen, f)
}

func ETestVectors(f io.Writer) {
	var key []uint8 = make([]uint8, MAXKEYLEN)
	var rkey []uint8 = make([]uint8, RNDS(MAXKEYLEN)*MAXBLOCKLEN)
	//var data [MAXBLOCKLEN]uint8
	//var cipher [MAXBLOCKLEN]uint8
	var blen_vals = [3]int{16, 32, 64}
	for blen_ind := 0; blen_ind < 3; blen_ind++ {
		var blen = blen_vals[blen_ind]
		for klen := MINKEYLEN; klen <= MAXKEYLEN; klen += KEYLENSTEP {
			fmt.Fprintf(f, "\n***** Key expansion, key len = %d *****\n", klen*8)
			KexpV(key, klen, blen, rkey, f)
			var data []uint8 = make([]uint8, MAXBLOCKLEN)
			var cipher []uint8 = make([]uint8, MAXBLOCKLEN)
			fmt.Fprintf(f, "\n***** Encryption, key len = %d, block len = %d *****\n", klen*8, blen*8)
			encryptV(data, rkey, klen, blen, cipher, f)
		}
	}
}

func ShortTestLin(f io.Writer, src io.Reader) {
	const BLEN_CNT = 3
	var din []uint8 = make([]uint8, MAXBLOCKLEN)
	var dout []uint8 = make([]uint8, MAXBLOCKLEN)
	if src == nil {
		for i := 0; i < MAXBLOCKLEN; i++ {
			din[i] = rnext()
		}
	} else {
		io.ReadAtLeast(src, din, MAXBLOCKLEN)
	}

	var blen_vals = [BLEN_CNT]int{16, 32, 64}
	fmt.Fprintf(f, "Linear operation:\n")
	for ex := 0; ex < BLEN_CNT; ex++ {
		fmt.Fprintf(f, "Input (%d bits):  ", blen_vals[ex]*8)
		for i := 0; i < blen_vals[ex]; i++ {
			fmt.Fprintf(f, "%02x", din[i])
		}
		fmt.Fprintf(f, "\nOutput (%d bits): ", blen_vals[ex]*8)
		linOp(din, dout, blen_vals[ex])
		for i := 0; i < blen_vals[ex]; i++ {
			fmt.Fprintf(f, "%02x", dout[i])
		}
		fmt.Fprintf(f, "\n")
	}
	fmt.Fprintf(f, "\n")
}

func ShortTestSBox(f io.Writer, src io.Reader) {
	var din []uint8 = make([]uint8, MAXBLOCKLEN)
	var dout []uint8 = make([]uint8, MAXBLOCKLEN)
	if src == nil {
		for i := 0; i < MAXBLOCKLEN; i++ {
			din[i] = rnext()
			dout[i] = sb[din[i]]
		}
	} else {
		io.ReadAtLeast(src, din, MAXBLOCKLEN)
		for i := 0; i < MAXBLOCKLEN; i++ {
			dout[i] = sb[din[i]]
		}
	}
	fmt.Fprintf(f, "Nonlinear operation:\n")
	fmt.Fprintf(f, "Input (%d bits):  ", MAXBLOCKLEN*8)
	for i := 0; i < MAXBLOCKLEN; i++ {
		fmt.Fprintf(f, "%02x", din[i])
	}
	fmt.Fprintf(f, "\nOutput (%d bits): ", MAXBLOCKLEN*8)
	for i := 0; i < MAXBLOCKLEN; i++ {
		fmt.Fprintf(f, "%02x", dout[i])
	}
	fmt.Fprintf(f, "\n\n")
}

func ShortTestKExp(f io.Writer, src io.Reader) {
	var key []uint8 = make([]uint8, MAXKEYLEN)
	var rkey []uint8 = make([]uint8, RNDS(MAXKEYLEN)*MAXBLOCKLEN)
	if src == nil {
		for i := 0; i < MAXKEYLEN; i++ {
			key[i] = rnext()
		}
	} else {
		io.ReadAtLeast(src, key, MAXKEYLEN)
	}

	fmt.Fprintf(f, "\nKey expansion for %d bit key and %d bit block:\n", MAXKEYLEN*8, MAXBLOCKLEN*8)
	KexpVV(key, MAXKEYLEN, MAXBLOCKLEN, rkey, f)
	fmt.Fprintf(f, "\n")
}

func ShortTestBasicEnc(f io.Writer, src io.Reader) {
	var key []uint8 = make([]uint8, MINKEYLEN)
	var rkey []uint8 = make([]uint8, RNDS(MINKEYLEN)*MINBLOCKLEN)
	var data []uint8 = make([]uint8, MINKEYLEN)
	var cipher []uint8 = make([]uint8, MINKEYLEN)
	if src == nil {
		for i := 0; i < MINBLOCKLEN; i++ {
			data[i] = rnext()
		}
		for i := 0; i < MINKEYLEN; i++ {
			key[i] = rnext()
		}
	} else {
		io.ReadAtLeast(src, data, MINKEYLEN)
		io.ReadAtLeast(src, key, MINKEYLEN)
	}

	fmt.Fprintf(f, "Encryption of 128 bit block and 256 bit key\n")
	Kexp(key, MINKEYLEN, MINBLOCKLEN, rkey)
	encryptV(data, rkey, MINKEYLEN, MINBLOCKLEN, cipher, f)
	fmt.Fprintf(f, "\n")
}

func ShortTestEnc(f io.Writer, src io.Reader) {
	var key []uint8 = make([]uint8, MAXKEYLEN)
	var rkey []uint8 = make([]uint8, RNDS(MAXKEYLEN)*MAXBLOCKLEN)
	var data []uint8 = make([]uint8, MAXBLOCKLEN)
	//var cipher []uint8 = make([]uint8, MAXBLOCKLEN)
	var blen_vals = [3]int{16, 32, 64}
	if src == nil {
		for i := 0; i < MAXBLOCKLEN; i++ {
			data[i] = rnext()
		}
		for i := 0; i < MAXKEYLEN; i++ {
			key[i] = rnext()
		}
	} else {
		io.ReadAtLeast(src, data, MAXBLOCKLEN)
		io.ReadAtLeast(src, key, MAXKEYLEN)
	}

	for klen := MINKEYLEN; klen <= MAXKEYLEN; klen += KEYLENSTEP {
		fmt.Fprintf(f, "\n***** Key expansion, key length = %d, block length = %d *****\n", klen*8, MINBLOCKLEN*8)
		KexpV(key, klen, MINBLOCKLEN, rkey, f)
		fmt.Fprintf(f, "\n")
	}

	for blen_ind := 0; blen_ind < 3; blen_ind++ {
		var blen = blen_vals[blen_ind]
		var data []uint8 = make([]uint8, MAXBLOCKLEN)
		var cipher []uint8 = make([]uint8, MAXBLOCKLEN)
		Kexp(key, MINKEYLEN, blen, rkey)
		fmt.Fprintf(f, "\n***** Encryption, key length = %d, block len = %d *****\n", MINKEYLEN*8, blen*8)
		encryptV(data, rkey, MINKEYLEN, blen, cipher, f)
		fmt.Fprintf(f, "\n")
	}
}

func ShortTestVectors(f io.Writer, src io.Reader) {
	ShortTestLin(f, src)
	ShortTestLin(f, src)
	ShortTestSBox(f, src)
	ShortTestKExp(f, src)
	//ShortTestBasicEnc(f);
	ShortTestEnc(f, src)
}
