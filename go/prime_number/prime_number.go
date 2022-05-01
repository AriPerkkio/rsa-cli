package prime_number

import "math"

// TODO
func GetTwoPrimeNumbers(bitLength int) (int, int) {
	return bitLength, bitLength
}

// TODO
func getPrimeNumber(bitLength int) int {
	return bitLength
}

func isPrime(number int) bool {
	if number < 2 {
		return false
	}

	if number%2 == 0 {
		return false
	}

	if number == 3 {
		return true
	}

	var square = int(math.Round(math.Sqrt(float64(number))))
	var i = 3

	for i < square {
		if number%i == 0 {
			return false
		}

		i += 2
	}

	return true
}
