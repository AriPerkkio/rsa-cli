mod prime_number;
mod rsa;

const BIT_LENGTH: u64 = 16;

fn main() {
    let (p, q) = prime_number::get_two_prime_numbers(BIT_LENGTH);
    println!("Prime numbers {} and {}", p, q);

    let keys = rsa::calculate_keys(&p, &q);

    let encrypted = rsa::encrypt("Hello world", &keys.public_key, &keys.public_exponent);
    println!("Encrypted: {}", rsa::display_as_text(&encrypted));

    let decrypted = rsa::decrypt(&encrypted, &keys.public_key, &keys.private_key);
    println!("Decrypted: {}", rsa::display_as_text(&decrypted));
}
