use num_bigint::{BigInt, RandBigInt, ToBigInt};

const BIT_LENGTH: u16 = 96;

fn main() {
    loop {
        let mut rng = rand::thread_rng();
        let number = rng.gen_bigint(BIT_LENGTH as u64);

        if is_prime(&number) {
            println!("Rust ready, {}", number);
            break;
        }
    }
}

fn is_prime(number: &BigInt) -> bool {
    let big_int_zero = ToBigInt::to_bigint(&0).unwrap();
    let big_int_two = ToBigInt::to_bigint(&2).unwrap();
    let big_int_three = ToBigInt::to_bigint(&3).unwrap();

    if number < &big_int_two {
        return false;
    }

    if number % 2 == big_int_zero {
        return false;
    }

    if number == &big_int_three {
        return true;
    }

    let mut i = ToBigInt::to_bigint(&3).unwrap();
    let square = number.sqrt();

    while i < square {
        i += 2;

        if number % &i == big_int_zero {
            return false;
        }
    }

    return true;
}
