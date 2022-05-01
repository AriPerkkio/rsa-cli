import getPrimeNumber from './prime-number';
import { encrypt, decrypt, displayAsText, calculateKeys } from './rsa';

const BIT_LENGTH = 8;

const p = getPrimeNumber(BIT_LENGTH);
const q = getPrimeNumber(BIT_LENGTH);

console.log('Prime numbers', p, q);

const { privateKey, publicKey, publicExponent } = calculateKeys(p, q);

const encrypted = encrypt('Hello world', publicKey, publicExponent);
console.log('Encypted', displayAsText(encrypted));

const decrypted = decrypt(encrypted, publicKey, privateKey);
console.log('Decypted', displayAsText(decrypted));
