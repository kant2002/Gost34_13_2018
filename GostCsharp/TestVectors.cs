namespace GostCsharp;
using static GostCsharp.Qalqan;
using static GostCsharp.PRNG;

internal static class TestVectors
{
    static unsafe void Pr(byte* b, int len, StreamWriter f)
    {
        for (int i = 0; i < len; i++)
            f.Write("{0:x2}", b[i]);
        f.Write("\n");
    }
    static unsafe void Pr(string str, byte* b, int len, StreamWriter f)
    {
        f.Write("{0}:\n", str);
        for (int i = 0; i < len; i++)
            f.Write("{0:x2}", b[i]);
        f.Write("\n");
        f.Flush();
    }

    static unsafe void KexpV(byte* key, int klen, int blen, byte* rkey, StreamWriter f)
    {
        Pr("Initial key", key, klen, f);
        byte* r0 = stackalloc byte[17], r1 = stackalloc byte[15];
        int addk = klen - 32, step = 0, s = SHIFT;
        for (int i = 0; i < 15; i++)
        {
            r0[i] = key[2 * i];
            r1[i] = key[2 * i + 1];
        }
        r0[15] = key[30];
        r0[16] = key[31];
        Pr("Register L0", r0, 17, f);
        Pr("Register L1", r1, 15, f);
        for (int r = 0; r < RNDS(klen); r++)
        {
            for (int k = 0; k < blen + s; k++)
            {
                byte t0 = (byte)(sb[r0[0]] + r0[1] + sb[r0[3]] + r0[7] + sb[r0[12]] + r0[16]);
                byte t1 = (byte)(sb[r1[0]] + r1[3] + sb[r1[9]] + r1[12] + sb[r1[14]]);
                for (int i = 0; i < 14; i++)
                {
                    r0[i] = r0[i + 1];
                    r1[i] = r1[i + 1];
                }
                r0[14] = r0[15];
                r0[15] = r0[16];
                if (k >= s)
                {
                    rkey[r * blen + k - s] = (byte)(t0 + r1[4]);
                    if (step < addk)
                    {
                        if ((step & 1) != 0)
                        {
                            //f.Write("Additional key byte to L0: {0:x2}\n", key[32 + step]);
                            t0 += key[32 + step];
                        }
                        else
                        {
                            //f.Write("Additional key byte to L1: {0:x2}\n", key[32 + step]);
                            t1 += key[32 + step];
                        }
                        step++;
                    }
                    //if (r < 2)
                    //f.Write("Output {0:x2} (L0: {1:x2}, L1: {2:x2})\n", (t0 + r1[4]) & 0xff, t0, r1[4]);
                }
                r0[16] = t0;
                r1[14] = t1;
                /*if (step < addk || (!r && (k < s)))
                {
                    f.Write("Step {0}\nFeedback L0: {1:x2}, L1: {2:x2}\n", k, t0, t1);
                    Pr("Register L0", r0, 17, f);
                    Pr("Register L1", r1, 15, f);
                }*/
            }
            if (s != 0)
            {
                f.Write("After prerun:\n");
                Pr("Register L0", r0, 17, f);
                Pr("Register L1", r1, 15, f);
            }
            s = 0;
        }
        for (int r = 0; r < RNDS(klen); r++)
        {
            f.Write("Round {0,2} key: ", r);
            Pr(rkey + blen * r, blen, f);
        }
    }

    static unsafe void KexpVV(byte* key, int klen, int blen, byte* rkey, StreamWriter f)
    {
        Pr("Initial key", key, klen, f);
        byte* r0 = stackalloc byte[17], r1 = stackalloc byte[15];
        int addk = klen - 32, step = 0, s = SHIFT;
        for (int i = 0; i < 15; i++)
        {
            r0[i] = key[2 * i];
            r1[i] = key[2 * i + 1];
        }
        r0[15] = key[30];
        r0[16] = key[31];
        Pr("Register L0", r0, 17, f);
        Pr("Register L1", r1, 15, f);
        for (int r = 0; r < RNDS(klen); r++)
        {
            for (int k = 0; k < blen + s; k++)
            {
                byte t0 = (byte)(sb[r0[0]] + r0[1] + sb[r0[3]] + r0[7] + sb[r0[12]] + r0[16]);
                byte t1 = (byte)(sb[r1[0]] + r1[3] + sb[r1[9]] + r1[12] + sb[r1[14]]);
                for (int i = 0; i < 14; i++)
                {
                    r0[i] = r0[i + 1];
                    r1[i] = r1[i + 1];
                }
                r0[14] = r0[15];
                r0[15] = r0[16];
                if (k >= s)
                {
                    rkey[r * blen + k - s] = (byte)(t0 + r1[4]);
                    if (step < addk)
                    {
                        if ((step & 1) != 0)
                        {
                            if (step < 32)
                                f.Write("Additional key byte to L0: {0:x2}\n", key[32 + step]);
                            t0 += key[32 + step];
                        }
                        else
                        {
                            if (step < 32)
                                f.Write("Additional key byte to L1: {0:x2}\n", key[32 + step]);
                            t1 += key[32 + step];
                        }
                        if (step < 32)
                        {
                            Pr("Register L0", r0, 17, f);
                            Pr("Register L1", r1, 15, f);
                        }
                        step++;
                    }
                    if (step < 8)
                        f.Write("Output {0:x2} (L0: {1:x2}, L1: {2:x2})\n", (t0 + r1[4]) & 0xff, t0, r1[4]);
                }
                r0[16] = t0;
                r1[14] = t1;
                //if (step < addk || (!r && (k < s)))
                if (step < 16)
                {
                    f.Write("Step {0}\nFeedback L0: {1:x2}, L1: {2:x2}\n", k, t0, t1);
                    Pr("Register L0", r0, 17, f);
                    Pr("Register L1", r1, 15, f);
                }
            }
            s = 0;
        }
        for (int r = 0; r < RNDS(klen); r++)
        {
            f.Write("Round {0,2} key: ", r);
            Pr(rkey + blen * r, blen, f);
        }
    }

    static unsafe void encryptV(byte* data, byte* rkey, int klen, int blen, byte* res, StreamWriter f)
    {
        byte* block = stackalloc byte[MAXBLOCKLEN], block2 = stackalloc byte[MAXBLOCKLEN];
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
            f.Write("Round {0}.\nKey{0}:\n", i, i);
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

    static unsafe void ETestVectors(StreamWriter f)
    {
        byte* key = stackalloc byte[MAXKEYLEN], rkey = stackalloc byte[RNDS(MAXKEYLEN) * MAXBLOCKLEN];
        byte* data = stackalloc byte[MAXBLOCKLEN], cipher = stackalloc byte[MAXBLOCKLEN];
        ReadOnlySpan<int> blen_vals = [16, 32, 64];
        for (int blen_ind = 0; blen_ind < 3; blen_ind++)
        {
            int blen = blen_vals[blen_ind];
            for (int klen = MINKEYLEN; klen <= MAXKEYLEN; klen += KEYLENSTEP)
            {
                f.Write("\n***** Key expansion, key len = {0} *****\n", klen * 8);
                KexpV(key, klen, blen, rkey, f);
                byte* data2 = stackalloc byte[MAXBLOCKLEN], cipher2 = stackalloc byte[MAXBLOCKLEN];
                f.Write("\n***** Encryption, key len = {0}, block len = {1} *****\n", klen * 8, blen * 8);
                encryptV(data2, rkey, klen, blen, cipher2, f);
            }
        }
    }

    static unsafe void ShortTestLin(StreamWriter f, FileStream src)
    {
        const int BLEN_CNT = 3;
        byte* din = stackalloc byte[MAXBLOCKLEN], dout = stackalloc byte[MAXBLOCKLEN];
        if (src == null)
        {
            for (int i = 0; i < MAXBLOCKLEN; i++)
                din[i] = rnext();
        }
        else
        {
            fread_s(din, MAXBLOCKLEN, sizeof(byte), MAXBLOCKLEN, src);
        }

        ReadOnlySpan<int> blen_vals = [16, 32, 64];
        f.Write("Linear operation:\n");
        for (int ex = 0; ex < BLEN_CNT; ex++)
        {
            f.Write("Input ({0} bits):  ", blen_vals[ex] * 8);
            for (int i = 0; i < blen_vals[ex]; i++)
                f.Write("{0:x2}", din[i]);
            f.Write("\nOutput ({0} bits): ", blen_vals[ex] * 8);
            linOp(din, dout, blen_vals[ex]);
            for (int i = 0; i < blen_vals[ex]; i++)
                f.Write("{0:x2}", dout[i]);
            f.Write("\n");
        }
        f.Write("\n");
    }

    static unsafe void fread_s(byte* din, int bufferSize, int size, int count, FileStream src)
    {
        for (var i = 0; i < count; i++)
        {
            din[i] = (byte)src.ReadByte();
        }
    }

    static unsafe void ShortTestSBox(StreamWriter f, FileStream src)
    {
        byte* din = stackalloc byte[MAXBLOCKLEN], dout = stackalloc byte[MAXBLOCKLEN];
        if (src == null)
        {
            for (int i = 0; i < MAXBLOCKLEN; i++)
            {
                din[i] = rnext();
                dout[i] = sb[din[i]];
            }
        }
        else
        {
            fread_s(din, MAXBLOCKLEN, sizeof(byte), MAXBLOCKLEN, src);
            for (int i = 0; i < MAXBLOCKLEN; i++)
            {
                dout[i] = sb[din[i]];
            }
        }
        f.Write("Nonlinear operation:\n");
        f.Write("Input ({0} bits):  ", MAXBLOCKLEN * 8);
        for (int i = 0; i < MAXBLOCKLEN; i++)
            f.Write("{0:x2}", din[i]);
        f.Write("\nOutput ({0} bits): ", MAXBLOCKLEN * 8);
        for (int i = 0; i < MAXBLOCKLEN; i++)
            f.Write("{0:x2}", dout[i]);
        f.Write("\n\n");
    }

    static unsafe void ShortTestKExp(StreamWriter f, FileStream src)
    {
        byte* key = stackalloc byte[MAXKEYLEN], rkey = stackalloc byte[RNDS(MAXKEYLEN) * MAXBLOCKLEN];
        if (src == null)
        {
            for (int i = 0; i < MAXKEYLEN; i++)
                key[i] = rnext();
        }
        else
        {
            fread_s(key, MAXKEYLEN, sizeof(byte), MAXKEYLEN, src);
        }

        f.Write("\nKey expansion for {0} bit key and {1} bit block:\n", MAXKEYLEN * 8, MAXBLOCKLEN * 8);
        KexpVV(key, MAXKEYLEN, MAXBLOCKLEN, rkey, f);
        f.Write("\n");
    }

    static unsafe void ShortTestBasicEnc(StreamWriter f, FileStream src)
    {
        byte* key = stackalloc byte[MINKEYLEN], rkey = stackalloc byte[RNDS(MINKEYLEN) * MINBLOCKLEN];
        byte* data = stackalloc byte[MINBLOCKLEN], cipher = stackalloc byte[MINBLOCKLEN];

        if (src != null)
        {
            for (int i = 0; i < MINBLOCKLEN; i++)
                data[i] = rnext();

            for (int i = 0; i < MINKEYLEN; i++)
                key[i] = rnext();
        }
        else
        {
            fread_s(data, MINBLOCKLEN, sizeof(byte), MINBLOCKLEN, src);
            fread_s(key, MINKEYLEN, sizeof(byte), MINKEYLEN, src);
        }

        f.Write("Encryption of 128 bit block and 256 bit key\n");
        Kexp(key, MINKEYLEN, MINBLOCKLEN, rkey);
        encryptV(data, rkey, MINKEYLEN, MINBLOCKLEN, cipher, f);
        f.Write("\n");
    }

    static unsafe void ShortTestEnc(StreamWriter f, FileStream src)
    {
        byte* key = stackalloc byte[MAXKEYLEN], rkey = stackalloc byte[RNDS(MAXKEYLEN) * MAXBLOCKLEN];
        byte* data = stackalloc byte[MAXBLOCKLEN], cipher = stackalloc byte[MAXBLOCKLEN];
        ReadOnlySpan<int> blen_vals = [16, 32, 64];

        if (src == null)
        {
            for (int i = 0; i < MAXBLOCKLEN; i++)
                data[i] = rnext();

            for (int i = 0; i < MAXKEYLEN; i++)
                key[i] = rnext();
        }
        else
        {
            fread_s(data, MAXBLOCKLEN, sizeof(byte), MAXBLOCKLEN, src);
            fread_s(key, MAXKEYLEN, sizeof(byte), MAXKEYLEN, src);
        }

        for (int klen = MINKEYLEN; klen <= MAXKEYLEN; klen += KEYLENSTEP)
        {
            f.Write("\n***** Key expansion, key length = {0}, block length = {1} *****\n", klen * 8, MINBLOCKLEN * 8);
            KexpV(key, klen, MINBLOCKLEN, rkey, f);
            f.Write("\n");
        }

        for (int blen_ind = 0; blen_ind < 3; blen_ind++)
        {
            int blen = blen_vals[blen_ind];
            byte* data2 = stackalloc byte[MAXBLOCKLEN], cipher2 = stackalloc byte[MAXBLOCKLEN];
            Kexp(key, MINKEYLEN, blen, rkey);
            f.Write("\n***** Encryption, key length = {0}, block len = {1} *****\n", MINKEYLEN * 8, blen * 8);
            encryptV(data2, rkey, MINKEYLEN, blen, cipher2, f);
            f.Write("\n");
        }
    }

    public static void ShortTestVectors(StreamWriter f, FileStream src)
    {
        ShortTestLin(f, src);
        ShortTestLin(f, src);
        ShortTestSBox(f, src);
        ShortTestKExp(f, src);
        //ShortTestBasicEnc(f);
        ShortTestEnc(f, src);
    }

}
