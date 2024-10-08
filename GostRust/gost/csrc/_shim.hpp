#ifndef __XXX_GOST_CXX_CXX_SHIM_HPP__
#define __XXX_GOST_CXX_CXX_SHIM_HPP__

#include "Qalqan.h"

extern "C" void shim_init(uint32_t ix);
extern "C" uint8_t shim_rnext();

extern "C" void shim_linOp(void* d, void* r, int blocklen);

extern "C" void shim_lin344(uint32_t* din, uint32_t* dout);
extern "C" void shim_lin384(uint32_t* din, uint32_t* dout);
extern "C" void shim_lin388(uint64_t* din, uint64_t* dout);

#endif // __XXX_GOST_CXX_CXX_SHIM_HPP__