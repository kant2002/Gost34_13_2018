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

extern "C" void shim_sBox(uint8_t* data, uint8_t* res, int blocklen)
{
    sBox(data, res, blocklen);
}
extern "C" void shim_AddRkX(uint8_t* block, uint8_t* rkey, int nr, int blen, uint8_t* res)
{
    AddRkX(block, rkey, nr, blen, res);
}
extern "C" void shim_AddRk(uint8_t* block, uint8_t* rkey, int nr, int blen, uint8_t* res)
{
    AddRk(block, rkey, nr, blen, res);
}
extern "C" void shim_encrypt(uint8_t* data, uint8_t* rkey, int klen, int blen, uint8_t* res)
{
    encrypt(data, rkey, klen, blen, res);
}

extern "C" void shim_decrypt(uint8_t* data, uint8_t* rkey, int klen, int blen, uint8_t* res)
{
    decrypt(data, rkey, klen, blen, res);
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

extern "C" void shim_Kexp(uint8_t* key, int klen, int blen, uint8_t* rkey)
{
    Kexp(key, klen, blen, rkey);
}