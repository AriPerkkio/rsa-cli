use num_bigint::{BigInt, RandBigInt, ToBigInt};

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
    let number = rng.gen_bigint(bit_length);

    if is_prime(&number) {
        return number;
    }

    return get_prime_number(bit_length);
}

fn is_prime(number: &BigInt) -> bool {
    let big_int_zero = 0.to_bigint().unwrap();
    let big_int_two = 2.to_bigint().unwrap();
    let big_int_three = 3.to_bigint().unwrap();

    if number < &big_int_two {
        return false;
    }

    if number % 2 == big_int_zero {
        return false;
    }

    if number == &big_int_three {
        return true;
    }

    let mut i = 3.to_bigint().unwrap();
    let square = number.sqrt();

    while i < square {
        if number % &i == big_int_zero {
            return false;
        }

        i += 2;
    }

    return true;
}
