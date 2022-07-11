use rand::prelude::*;

pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_pow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_pow(b_pub, a, p)
}

fn modular_pow(base: u64, exponent: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }

    let mut result = 1;
    let mut exponent = exponent as u128;
    let mut base = base as u128;
    let m = modulus as u128;

    assert_ne!((m - 1).checked_mul(m - 1), None);

    base %= m;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % m;
        }
        exponent >>= 1;
        base = (base * base) % m;
    }

    result as u64 % modulus
}

fn unoptimized_modular_pow(base: u64, exponent: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut c = 1_u64;
    for _ in 0..=(exponent - 1) {
        c = (c * base) % modulus
    }
    c
}
