use num_bigint::{BigInt, ToBigInt};
use num_traits::{one, zero};

use crate::prime_number;

#[derive(Debug)]
pub struct Keys {
    pub public_key: BigInt,
    pub public_exponent: BigInt,
    pub private_key: BigInt,
}

pub fn encrypt(message: &str, public_key: &BigInt, exponent: &BigInt) -> Vec<BigInt> {
    message
        .chars()
        .map(|character| (character as u32).to_bigint().unwrap())
        .map(|num| num.modpow(&exponent, &public_key))
        .collect()
}

pub fn decrypt(message: &Vec<BigInt>, public_key: &BigInt, private_key: &BigInt) -> Vec<BigInt> {
    message
        .iter()
        .map(|character| character.modpow(&private_key, &public_key))
        .collect()
}

pub fn calculate_keys(p: &BigInt, q: &BigInt) -> Keys {
    let n = p * q;
    let f: BigInt = (p - 1) * (q - 1);

    let mut e = prime_number::get_prime_number(f.bits());

    let mut gcd: BigInt;
    let mut y_factor: BigInt;

    loop {
        e += 1;

        let output = calculate_greatest_common_divisor(&f, &e);
        gcd = output.0;
        y_factor = output.1;

        if gcd == one() {
            break;
        }
    }

    Keys {
        public_key: n,
        public_exponent: e,
        private_key: y_factor,
    }
}

fn calculate_greatest_common_divisor(_x: &BigInt, _y: &BigInt) -> (BigInt, BigInt) {
    let mut x = _x.clone();
    let mut y = _y.clone();

    let mut factor: BigInt;
    let mut leftover: BigInt;

    let mut z: BigInt;
    let mut xx: BigInt = one();
    let mut yy: BigInt = zero();
    let mut v: BigInt = one();
    let mut u: BigInt = zero();

    loop {
        factor = &x / &y;
        leftover = &x - &factor * &y;

        // Setup for next iteration
        x = y;
        y = leftover;

        // Calculate Y-factor
        z = u.clone();
        u = &xx - &factor * &u;
        xx = z;
        z = v.clone();
        v = &yy - &factor * &v;
        yy = z;

        if y == zero() {
            break;
        }
    }

    return (x, if yy < zero() { _x + yy } else { yy });
}

pub fn display_as_text(numeric: &Vec<BigInt>) -> String {
    let mut textual = String::new();

    for numeric_char in numeric {
        for digit in numeric_char.to_bytes_be().1 {
            textual.push(digit as char)
        }
    }

    return textual.escape_default().collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_gcd() {
        let (gcd, _) =
            calculate_greatest_common_divisor(&27.to_bigint().unwrap(), &18.to_bigint().unwrap());

        assert_eq!(gcd, 9.to_bigint().unwrap());
    }
}
