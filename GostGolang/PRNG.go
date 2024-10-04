package main

var x uint32 = 1

func initrng(ix uint32) {
	x = ix
}

func rnext() uint8 { //ISO/IEC 9899:201x p347 (http://www.open-std.org/jtc1/sc22/wg14/www/docs/n1570.pdf)
	//return (x = (1103515245 * x + 12345)) >> 16;
	x = x*1103515245 + 12345
	return uint8((x / 65536) % 32768)
}
