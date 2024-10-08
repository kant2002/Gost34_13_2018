#include "_shim.hpp"

void lin344(uint32_t* din, uint32_t* dout);
void lin384(uint32_t* din, uint32_t* dout);
void lin388(uint64_t* din, uint64_t* dout);

extern "C" void shim_init(uint32_t ix)
{
    init(ix);
}

extern "C" uint8_t shim_rnext()
{
    return rnext();
}

extern "C" void shim_linOp(void* d, void* r, int blocklen)
{
    linOp(d, r, blocklen);
}

extern "C" void shim_lin344(uint32_t* din, uint32_t* dout)
{
    lin344(din, dout);
}
extern "C" void shim_lin384(uint32_t* din, uint32_t* dout)
{
    lin384(din, dout);
}
extern "C" void shim_lin388(uint64_t* din, uint64_t* dout)
{
    lin388(din, dout);
}