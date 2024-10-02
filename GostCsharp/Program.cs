using static GostCsharp.PRNG;
using static GostCsharp.TestVectors;

init(0);
var src = File.OpenRead("../../../../resources/input.bin");
var f = new StreamWriter(new FileStream("../../../../resources/output_charp.txt", FileMode.Create));
ShortTestVectors(f, src);
