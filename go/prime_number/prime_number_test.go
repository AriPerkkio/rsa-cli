package prime_number

import (
	"testing"
)

func TestPrimeNumbersAreIdentified(t *testing.T) {
	if isPrime(5) == false {
		t.Fatalf("5 is prime")
	}
	if isPrime(131) == false {
		t.Fatalf("13 is prime")
	}
}

func TestNonPrimeNumbersAreIdentified(t *testing.T) {
	if isPrime(16) == true {
		t.Fatalf("16 is not prime")
	}
	if isPrime(13124) == true {
		t.Fatalf("13124 is not prime")
	}
}
