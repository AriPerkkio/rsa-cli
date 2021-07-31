import getPrimeNumber from './prime-number';

export function encrypt(
    message: string,
    publicKey: bigint,
    exponent: bigint
): bigint[] {
    const encrypted: bigint[] = [];

    for (let i = 0; i < message.length; i++) {
        encrypted.push(
            bigIntModPow(BigInt(message.charCodeAt(i)), exponent, publicKey)
        );
    }

    return encrypted;
}

export function decrypt(
    message: bigint[],
    publicKey: bigint,
    privateKey: bigint
): bigint[] {
    const decrypted: bigint[] = [];

    for (const character of message) {
        decrypted.push(bigIntModPow(character, privateKey, publicKey));
    }

    return decrypted;
}

function bigIntModPow(
    number: bigint,
    exponent: bigint,
    modulus: bigint
): bigint {
    let result = 1n;

    for (let i = 0n; i < exponent; i++) {
        result = result * number;
        result = result % modulus;
    }

    return result;
}

export function calculateKeys(
    p: bigint,
    q: bigint
): {
    publicKey: bigint;
    publicExponent: bigint;
    privateKey: bigint;
} {
    const n = p * q;
    const f = (p - 1n) * (q - 1n);

    let e = getPrimeNumber(f.toString(2).length);
    let gcd = 0n;
    let yFactor: bigint;

    do {
        e += 1n;
        ({ gcd, yFactor } = calculateGreatestCommonDivisor(f, e));
    } while (gcd !== 1n);

    return {
        publicKey: n,
        publicExponent: e,
        privateKey: yFactor,
    };
}

function calculateGreatestCommonDivisor(
    _x: bigint,
    _y: bigint
): { gcd: bigint; yFactor: bigint } {
    let x = _x;
    let y = _y;

    let factor: bigint;
    let leftover: bigint;

    let z: bigint;
    let xx = 1n;
    let yy = 0n;
    let v = 1n;
    let u = 0n;

    while (y !== 0n) {
        factor = x / y;
        leftover = x - factor * y;

        // Setup for next iteration
        x = y;
        y = leftover;

        // Calculate Y-factor
        z = u;
        u = xx - factor * u;
        xx = z;
        z = v;
        v = yy - factor * v;
        yy = z;
    }

    return { gcd: x, yFactor: yy < 0 ? _x + yy : yy };
}

export function displayAsText(encrypted: bigint[]): string {
    return encrypted
        .map(e => String.fromCharCode(parseInt(e.toString())))
        .join('');
}
