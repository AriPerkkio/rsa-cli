package main

import (
	"fmt"
	"rsa-cli/prime_number"
)

func main() {
	var p, q = prime_number.GetTwoPrimeNumbers(8)

	fmt.Println(p, q)
}
