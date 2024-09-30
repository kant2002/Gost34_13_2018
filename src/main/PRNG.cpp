#include "Qalqan.h"

static uint32_t x = 1;

void init(uint32_t ix)
{
	x = ix;
}

uint8_t rnext()
{ //ISO/IEC 9899:201x p347 (http://www.open-std.org/jtc1/sc22/wg14/www/docs/n1570.pdf)
	//return (x = (1103515245 * x + 12345)) >> 16;
	x = x * 1103515245 + 12345;
	return (unsigned int)(x / 65536) % 32768;
}
