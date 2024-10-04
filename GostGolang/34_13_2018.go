package main

import (
	"log"
	"os"
)

//import "./PRNG"

func main() {
	initrng(0)
	src, err := os.Open("../resources/input.bin")
	if err != nil {
		log.Fatal(err)
	}

	f, err := os.OpenFile("../resources/output_go.txt", os.O_WRONLY|os.O_CREATE|os.O_TRUNC, 0644)
	if err != nil {
		log.Fatal(err)
	}

	ShortTestVectors(f, src)
	if err := f.Close(); err != nil {
		log.Fatal(err)
	}
	if err := src.Close(); err != nil {
		log.Fatal(err)
	}
}
