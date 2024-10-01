module PRNG

let mutable private x = 1

let init new_x =
    x <- new_x

let rnext () =
    x <- (1103515245 * x + 12345)
    byte (x >>> 16)

