#ifndef __XXX_GOST_CXX_CXX_SHIM_HPP__
#define __XXX_GOST_CXX_CXX_SHIM_HPP__

#include "Qalqan.h"

extern "C" void shim_init(uint32_t ix);
extern "C" uint8_t shim_rnext();

extern "C" void shim_linOp(void* d, void* r, int blocklen);
extern "C" void shim_sBox(uint8_t* data, uint8_t* res, int blocklen);
extern "C" void shim_AddRkX(uint8_t* block, uint8_t* rkey, int nr, int blen, uint8_t* res);
extern "C" void shim_AddRk(uint8_t* block, uint8_t* rkey, int nr, int blen, uint8_t* res);
extern "C" void shim_encrypt(uint8_t* data, uint8_t* rkey, int klen, int blen, uint8_t* res);

extern "C" void shim_decrypt(uint8_t* data, uint8_t* rkey, int klen, int blen, uint8_t* res);

extern "C" void shim_lin344(uint32_t* din, uint32_t* dout);
extern "C" void shim_lin384(uint32_t* din, uint32_t* dout);
extern "C" void shim_lin388(uint64_t* din, uint64_t* dout);

extern "C" void shim_Kexp(uint8_t* key, int klen, int blen, uint8_t* rkey);

//AddRk
//sBox
//AddRkX
//encrypt

//decrypt
#endif // __XXX_GOST_CXX_CXX_SHIM_HPP__