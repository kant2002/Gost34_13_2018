#include <cstdint>
#include <stdio.h>
#include <assert.h>

#define SHIFT 17
#define MINBLOCKLEN 16
#define MAXBLOCKLEN 64
#define MINKEYLEN 32
#define MAXKEYLEN 128
#define KEYLENSTEP 16
#define RNDS(x) (16+(x-32)/16)
#define ROTL(x,s) ((x<<s)|(x>>(32-s)))
#define ROTL64(x,s) ((x<<s)|(x>>(64-s))) 

// Key expansion
void Kexp(uint8_t* key, int klen, int blen, uint8_t* rkey);

// Encrypting
void linOp(void* d, void* r, int blocklen);
void sBox(uint8_t* data, uint8_t* res, int blocklen);
void AddRkX(uint8_t* block, uint8_t* rkey, int nr, int blen, uint8_t* res);
void AddRk(uint8_t* block, uint8_t* rkey, int nr, int blen, uint8_t* res);
void encrypt(uint8_t* data, uint8_t* rkey, int klen, int blen, uint8_t* res);

// Decrypting
void InvAddRk(uint8_t* block, uint8_t* rkey, int nr, int blen, uint8_t* res);
void InvlinOp(void* d, void* r, int blocklen);
void InvsBox(uint8_t* data, uint8_t* res, int blocklen);
void decrypt(uint8_t* data, uint8_t* rkey, int klen, int blen, uint8_t* res);

// PRNG
void init(uint32_t ix);
uint8_t rnext();	// (1103515245 * x + 12345 mod 2**32)[23..16]

// Tests
bool testAddRk();
bool testInvAddRk();
bool testInvSbox();
bool testInvLinOp();
bool testEncDec();
bool testEtalon();
bool TestFast();
bool testEncDec_32_16();

// Quality tests
void KeyAvalanche();
void KAvalAdditional(void f(uint8_t*, int, uint8_t*));
double SpeedTestKexp(void f(uint8_t*, int, int, uint8_t*), int klen, int blen);
double SpeedTestEnc(void f(uint8_t*, uint8_t*, int, int, uint8_t*), int klen, int blen);
double SpeedTestFastEnc_32_16();

// Test vectors generators
void KexpTestVectors(FILE* f);
void KexpV(uint8_t* key, int klen, int blen, uint8_t* rkey, FILE* f);
void KexpVV(uint8_t* key, int klen, int blen, uint8_t* rkey, FILE* f);
void ETestVectors(FILE* f);
void encryptV(uint8_t* data, uint8_t* rkey, int klen, int blen, uint8_t* res, FILE* f);

// Fast implementations
void encrypt_fast(uint8_t* data, uint8_t* rkey, int klen, int blen, uint8_t* res);
void Qalqan_encrypt_32_16(uint8_t* clear, uint8_t* cipher, uint8_t* rkey);
void Qalqan_encrypt_32_16v(uint8_t* clear, uint8_t* cipher, uint8_t* rkey);
void Qalqan_decrypt_32_16(uint8_t* cipher, uint8_t* clear, uint8_t* rkey);

void VerilogSBoxInit();
void ShortTestVectors(FILE* f);
