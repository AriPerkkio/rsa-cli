use num_bigint::{BigInt, RandBigInt, ToBigInt};
use num_traits::zero;

pub fn get_two_prime_numbers(bit_length: u64) -> (BigInt, BigInt) {
    let p = get_prime_number(bit_length);
    let q: BigInt = loop {
        let number = get_prime_number(bit_length);

        if number != p {
            break number;
        }
    };

    return (p, q);
}

pub fn get_prime_number(bit_length: u64) -> BigInt {
    let mut rng = rand::thread_rng();
    let mut number = rng.gen_bigint(bit_length);

    // Set MSB to 1 to ensure bit length
    number.set_bit(bit_length - 1, true);

    if is_prime(&number) {
        return number;
    }

    return get_prime_number(bit_length);
}

fn is_prime(number: &BigInt) -> bool {
    let big_int_two = 2.to_bigint().unwrap();
    let big_int_three = 3.to_bigint().unwrap();

    if number < &big_int_two {
        return false;
    }

    if number % 2 == zero() {
        return false;
    }

    if number == &big_int_three {
        return true;
    }

    let mut i = 3.to_bigint().unwrap();
    let square = number.sqrt();

    while i < square {
        if number % &i == zero() {
            return false;
        }

        i += 2;
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime_numbers_are_identified() {
        assert_eq!(is_prime(&5.to_bigint().unwrap()), true);
        assert_eq!(is_prime(&131.to_bigint().unwrap()), true);
    }

    #[test]
    fn non_prime_numbers_are_identified() {
        assert_eq!(is_prime(&16.to_bigint().unwrap()), false);
        assert_eq!(is_prime(&13124.to_bigint().unwrap()), false);
    }

    #[test]
    fn primes_match_given_bit_length() {
        let prime = get_prime_number(8);

        assert_eq!(prime.bits(), 8)
    }

    #[test]
    fn primes_are_unique() {
        let (p, q) = get_two_prime_numbers(4);

        assert!(p != q);
    }
}
