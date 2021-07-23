const BIT_LENGTH = 96;

function getPrimeNumber(bitCount = 2) {
    while (true) {
        const number = randomBigInt(bitCount);

        if (isPrimeNumber(number)) {
            return number;
        }
    }
}

function randomBigInt(bitLength) {
    const binaryString = Array(bitLength)
        .fill("1")
        .reduce((bin) => bin + Math.round(Math.random()).toString());

    return BigInt("0b" + binaryString);
}

function bigIntSquareRoot(number) {
    if (number < 0n) throw new Error("Negative numbers not supported");
    if (number < 2n) return number;

    function iterate(value, guess) {
        const nextGuess = (value / guess + guess) >> 1n;

        if (guess === nextGuess) return guess;
        if (guess === nextGuess - 1n) return guess;

        return iterate(value, nextGuess);
    }

    return iterate(number, 1n);
}

function isPrimeNumber(number) {
    if (number <= 2n) return false;
    if (number % 2n === 0n) return false;

    if (number === 3n) return true;

    const squareRoot = bigIntSquareRoot(number);

    for (let i = 3n; i < squareRoot; i += 2n) {
        if (number % i === 0n) {
            return false;
        }
    }

    return true;
}

console.log(`Node ready ${getPrimeNumber(BIT_LENGTH)}`);
