// https://ksm.kz/public-discussion/standards/discussion-of-draft-of-st-rk/112716/
#include <cstdio>
#include <Qalqan.h>

int main()
{
	init(0);
	FILE* src = fopen("resources/input.bin", "rb");
	FILE* f = fopen("resources/output.txt", "w");
	ShortTestVectors(f, src);
	return 0;
}