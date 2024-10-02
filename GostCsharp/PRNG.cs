namespace GostCsharp;

internal static class PRNG
{

    static int x = 1;

    public static void init(int ix)
    {
        x = ix;
    }

    public static byte rnext()
    { //ISO/IEC 9899:201x p347 (http://www.open-std.org/jtc1/sc22/wg14/www/docs/n1570.pdf)
      //return (x = (1103515245 * x + 12345)) >> 16;
        x = x * 1103515245 + 12345;
        return (byte)(uint)(x >> 16);
    }
}
