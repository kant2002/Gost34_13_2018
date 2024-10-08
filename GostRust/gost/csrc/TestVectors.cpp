#include "Qalqan.h"

extern uint8_t sb[256];

/*void Pr(uint8_t* b, int len)
{
    for (int i = 0; i < len; i++)
        printf("%02x", b[i]);
    printf("\n");
}
void Pr(const char*str, uint8_t*b, int len)
{
    printf("%s:\n", str);
    for (int i = 0; i < len; i++)
        printf("%02x", b[i]);
    printf("\n");
}
*/
void Pr(uint8_t* b, int len, FILE* f)
{
    for (int i = 0; i < len; i++)
        fprintf(f, "%02x", b[i]);
    fprintf(f, "\n");
}
void Pr(const char* str, uint8_t* b, int len, FILE* f)
{
    fprintf(f, "%s:\n", str);
    for (int i = 0; i < len; i++)
        fprintf(f, "%02x", b[i]);
    fprintf(f, "\n");
}

void KexpV(uint8_t* key, int klen, int blen, uint8_t* rkey, FILE* f)
{
    Pr("Initial key", key, klen, f);
    uint8_t r0[17], r1[15];
    int addk = klen - 32, step = 0, s = SHIFT;
    for (int i = 0; i < 15; i++)
    {
        r0[i] = key[2 * i];
        r1[i] = key[2 * i + 1];
    }
    r0[15] = key[30];
    r0[16] = key[31];
    Pr("Register L0\0", r0, 17, f);
    Pr("Register L1\0", r1, 15, f);
    for (int r = 0; r < RNDS(klen); r++)
    {
        for (int k = 0; k < blen + s; k++)
        {
            uint8_t t0 = sb[r0[0]] + r0[1] + sb[r0[3]] + r0[7] + sb[r0[12]] + r0[16];
            uint8_t t1 = sb[r1[0]] + r1[3] + sb[r1[9]] + r1[12] + sb[r1[14]];
            for (int i = 0; i < 14; i++)
            {
                r0[i] = r0[i + 1];
                r1[i] = r1[i + 1];
            }
            r0[14] = r0[15];
            r0[15] = r0[16];
            if (k >= s)
            {
                rkey[r * blen + k - s] = t0 + r1[4];
                if (step < addk)
                {
                    if (step & 1)
                    {
                        //fprintf(f, "Additional key byte to L0: %02x\n", key[32 + step]);
                        t0 += key[32 + step];
                    }
                    else
                    {
                        //fprintf(f, "Additional key byte to L1: %02x\n", key[32 + step]);
                        t1 += key[32 + step];
                    }
                    step++;
                }
                //if (r < 2)
                    //fprintf(f, "Output %02x (L0: %02x, L1: %02x)\n", (t0 + r1[4]) & 0xff, t0, r1[4]);
            }
            r0[16] = t0;
            r1[14] = t1;
            /*if (step < addk || (!r && (k < s)))
            {
                fprintf(f, "Step %d\nFeedback L0: %02x, L1: %02x\n", k, t0, t1);
                Pr("Register L0\0", r0, 17, f);
                Pr("Register L1\0", r1, 15, f);
            }*/
        }
        if (s)
        {
            fprintf(f, "After prerun:\n");
            Pr("Register L0\0", r0, 17, f);
            Pr("Register L1\0", r1, 15, f);
        }
        s = 0;
    }
    for (int r = 0; r < RNDS(klen); r++)
    {
        fprintf(f, "Round %2d key: ", r);
        Pr(rkey + blen * r, blen, f);
    }
}

void KexpVV(uint8_t* key, int klen, int blen, uint8_t* rkey, FILE* f)
{
    Pr("Initial key", key, klen, f);
    uint8_t r0[17], r1[15];
    int addk = klen - 32, step = 0, s = SHIFT;
    for (int i = 0; i < 15; i++)
    {
        r0[i] = key[2 * i];
        r1[i] = key[2 * i + 1];
    }
    r0[15] = key[30];
    r0[16] = key[31];
    Pr("Register L0\0", r0, 17, f);
    Pr("Register L1\0", r1, 15, f);
    for (int r = 0; r < RNDS(klen); r++)
    {
        for (int k = 0; k < blen + s; k++)
        {
            uint8_t t0 = sb[r0[0]] + r0[1] + sb[r0[3]] + r0[7] + sb[r0[12]] + r0[16];
            uint8_t t1 = sb[r1[0]] + r1[3] + sb[r1[9]] + r1[12] + sb[r1[14]];
            for (int i = 0; i < 14; i++)
            {
                r0[i] = r0[i + 1];
                r1[i] = r1[i + 1];
            }
            r0[14] = r0[15];
            r0[15] = r0[16];
            if (k >= s)
            {
                rkey[r * blen + k - s] = t0 + r1[4];
                if (step < addk)
                {
                    if (step & 1)
                    {
                        if (step < 32)
                            fprintf(f, "Additional key byte to L0: %02x\n", key[32 + step]);
                        t0 += key[32 + step];
                    }
                    else
                    {
                        if (step < 32)
                            fprintf(f, "Additional key byte to L1: %02x\n", key[32 + step]);
                        t1 += key[32 + step];
                    }
                    if (step < 32)
                    {
                        Pr("Register L0\0", r0, 17, f);
                        Pr("Register L1\0", r1, 15, f);
                    }
                    step++;
                }
                if (step < 8)
                    fprintf(f, "Output %02x (L0: %02x, L1: %02x)\n", (t0 + r1[4]) & 0xff, t0, r1[4]);
            }
            r0[16] = t0;
            r1[14] = t1;
            //if (step < addk || (!r && (k < s)))
            if (step < 16)
            {
                fprintf(f, "Step %d\nFeedback L0: %02x, L1: %02x\n", k, t0, t1);
                Pr("Register L0\0", r0, 17, f);
                Pr("Register L1\0", r1, 15, f);
            }
        }
        s = 0;
    }
    for (int r = 0; r < RNDS(klen); r++)
    {
        fprintf(f, "Round %2d key: ", r);
        Pr(rkey + blen * r, blen, f);
    }
}

void encryptV(uint8_t* data, uint8_t* rkey, int klen, int blen, uint8_t* res, FILE* f)
{
    uint8_t block[MAXBLOCKLEN] = { 0 }, block2[MAXBLOCKLEN] = { 0 };
    Pr("Clear text", data, blen, f);
    Pr("Key 0", rkey, blen, f);
    AddRk(data, rkey, 0, blen, block);
    Pr("After add K0", block, blen, f);
    sBox(block, block2, blen);
    Pr("After Sub", block2, blen, f);
    linOp(block2, block, blen);
    Pr("After linear", block, blen, f);
    for (int i = 1; i < RNDS(klen) - 1; i++)
    {
        fprintf(f, "Round %d.\nKey%d:\n", i, i);
        Pr(rkey + blen * i, blen, f);
        AddRkX(block, rkey, i, blen, block2);
        Pr("After addkey", block2, blen, f);
        sBox(block2, block2, blen);
        Pr("After Sub", block2, blen, f);
        linOp(block2, block, blen);
        Pr("After linear", block, blen, f);
    }
    Pr("Final key", rkey + blen * (RNDS(klen) - 1), blen, f);
    AddRk(block, rkey, RNDS(klen) - 1, blen, res);
    Pr("Ciphertext", res, blen, f);
}

void ETestVectors(FILE* f)
{
    uint8_t key[MAXKEYLEN] = { 0 }, rkey[RNDS(MAXKEYLEN) * MAXBLOCKLEN];
    uint8_t data[MAXBLOCKLEN] = { 0 }, cipher[MAXBLOCKLEN] = { 0 };
    int blen_vals[3] = { 16, 32, 64 };
    for (int blen_ind = 0; blen_ind < 3; blen_ind++)
    {
        int blen = blen_vals[blen_ind];
        for (int klen = MINKEYLEN; klen <= MAXKEYLEN; klen += KEYLENSTEP)
        {
            fprintf(f, "\n***** Key expansion, key len = %d *****\n", klen * 8);
            KexpV(key, klen, blen, rkey, f);
            uint8_t data[MAXBLOCKLEN] = { 0 }, cipher[MAXBLOCKLEN] = { 0 };
            fprintf(f, "\n***** Encryption, key len = %d, block len = %d *****\n", klen * 8, blen * 8);
            encryptV(data, rkey, klen, blen, cipher, f);
        }
    }
}

void ShortTestLin(FILE* f, FILE* src)
{
#define BLEN_CNT 3
    uint8_t din[MAXBLOCKLEN], dout[MAXBLOCKLEN];
    if (!src)
    {
        for (int i = 0; i < MAXBLOCKLEN; i++)
            din[i] = rnext();
    }
    else
    {
        fread_s(din, MAXBLOCKLEN, sizeof(uint8_t), MAXBLOCKLEN, src);
    }

    int blen_vals[BLEN_CNT] = { 16, 32, 64 };
    fprintf(f, "Linear operation:\n");
    for (int ex = 0; ex < BLEN_CNT; ex++)
    {
        fprintf(f, "Input (%d bits):  ", blen_vals[ex] * 8);
        for (int i = 0; i < blen_vals[ex]; i++)
            fprintf(f, "%02x", din[i]);
        fprintf(f, "\nOutput (%d bits): ", blen_vals[ex] * 8);
        linOp(din, dout, blen_vals[ex]);
        for (int i = 0; i < blen_vals[ex]; i++)
            fprintf(f, "%02x", dout[i]);
        fprintf(f, "\n");
    }
    fprintf(f, "\n");
#undef BLEN_CNT
}

void ShortTestSBox(FILE* f, FILE* src)
{
    uint8_t din[MAXBLOCKLEN], dout[MAXBLOCKLEN];
    if (!src)
    {
        for (int i = 0; i < MAXBLOCKLEN; i++)
        {
            din[i] = rnext();
            dout[i] = sb[din[i]];
        }
    }
    else
    {
        fread_s(din, MAXBLOCKLEN, sizeof(uint8_t), MAXBLOCKLEN, src);
        for (int i = 0; i < MAXBLOCKLEN; i++)
        {
            dout[i] = sb[din[i]];
        }
    }
    fprintf(f, "Nonlinear operation:\n");
    fprintf(f, "Input (%d bits):  ", MAXBLOCKLEN * 8);
    for (int i = 0; i < MAXBLOCKLEN; i++)
        fprintf(f, "%02x", din[i]);
    fprintf(f, "\nOutput (%d bits): ", MAXBLOCKLEN * 8);
    for (int i = 0; i < MAXBLOCKLEN; i++)
        fprintf(f, "%02x", dout[i]);
    fprintf(f, "\n\n");
}

void ShortTestKExp(FILE* f, FILE* src)
{
    uint8_t key[MAXKEYLEN] = { 0 }, rkey[RNDS(MAXKEYLEN) * MAXBLOCKLEN];
    if (!src)
    {
        for (int i = 0; i < MAXKEYLEN; i++)
            key[i] = rnext();
    }
    else
    {
        fread_s(key, MAXKEYLEN, sizeof(uint8_t), MAXKEYLEN, src);
    }

    fprintf(f, "\nKey expansion for %d bit key and %d bit block:\n", MAXKEYLEN * 8, MAXBLOCKLEN * 8);
    KexpVV(key, MAXKEYLEN, MAXBLOCKLEN, rkey, f);
    fprintf(f, "\n");
}

void ShortTestBasicEnc(FILE* f, FILE* src)
{
    uint8_t key[MINKEYLEN], rkey[RNDS(MINKEYLEN) * MINBLOCKLEN];
    uint8_t data[MINBLOCKLEN], cipher[MINBLOCKLEN] = { 0 };

    if (!src)
    {
        for (int i = 0; i < MINBLOCKLEN; i++)
            data[i] = rnext();

        for (int i = 0; i < MINKEYLEN; i++)
            key[i] = rnext();
    }
    else
    {
        fread_s(data, MINBLOCKLEN, sizeof(uint8_t), MINBLOCKLEN, src);
        fread_s(key, MINKEYLEN, sizeof(uint8_t), MINKEYLEN, src);
    }

    fprintf(f, "Encryption of 128 bit block and 256 bit key\n");
    Kexp(key, MINKEYLEN, MINBLOCKLEN, rkey);
    encryptV(data, rkey, MINKEYLEN, MINBLOCKLEN, cipher, f);
    fprintf(f, "\n");
}

void ShortTestEnc(FILE* f, FILE* src)
{
    uint8_t key[MAXKEYLEN] = { 0 }, rkey[RNDS(MAXKEYLEN) * MAXBLOCKLEN];
    uint8_t data[MAXBLOCKLEN], cipher[MAXBLOCKLEN] = { 0 };
    int blen_vals[3] = { 16, 32, 64 };

    if (!src)
    {
        for (int i = 0; i < MAXBLOCKLEN; i++)
            data[i] = rnext();

        for (int i = 0; i < MAXKEYLEN; i++)
            key[i] = rnext();
    }
    else
    {
        fread_s(data, MAXBLOCKLEN, sizeof(uint8_t), MAXBLOCKLEN, src);
        fread_s(key, MAXKEYLEN, sizeof(uint8_t), MAXKEYLEN, src);
    }

    for (int klen = MINKEYLEN; klen <= MAXKEYLEN; klen += KEYLENSTEP)
    {
        fprintf(f, "\n***** Key expansion, key length = %d, block length = %d *****\n", klen * 8, MINBLOCKLEN * 8);
        KexpV(key, klen, MINBLOCKLEN, rkey, f);
        fprintf(f, "\n");
    }

    for (int blen_ind = 0; blen_ind < 3; blen_ind++)
    {
        int blen = blen_vals[blen_ind];
        uint8_t data[MAXBLOCKLEN] = { 0 }, cipher[MAXBLOCKLEN] = { 0 };
        Kexp(key, MINKEYLEN, blen, rkey);
        fprintf(f, "\n***** Encryption, key length = %d, block len = %d *****\n", MINKEYLEN * 8, blen * 8);
        encryptV(data, rkey, MINKEYLEN, blen, cipher, f);
        fprintf(f, "\n");
    }
}

void ShortTestVectors(FILE* f, FILE* src)
{
    ShortTestLin(f, src);
    ShortTestLin(f, src);
    ShortTestSBox(f, src);
    ShortTestKExp(f, src);
    //ShortTestBasicEnc(f);
    ShortTestEnc(f, src);
}
