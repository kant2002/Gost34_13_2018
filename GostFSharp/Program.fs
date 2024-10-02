open PRNG
open TestVectors
open System.IO

[<EntryPoint>]
let main _ =
    init(0)

    let src = File.OpenRead("../../../../resources/input.bin")
    use f = new StreamWriter(new FileStream("../../../../resources/output_fsharp.txt", FileMode.Create))
    ShortTestVectors(f, src)
    f.Flush()
    f.BaseStream.Flush()
    0
