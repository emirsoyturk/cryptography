use cryptography::Field;

pub struct U64Field<const MODULUS: u64>;

impl<const MODULUS: u64> Field for U64Field<MODULUS> {
    type BaseType = u64;

    fn add(a: &u64, b: &u64) -> u64 {
        ((*a as u128 + *b as u128) % MODULUS as u128) as u64
    }

    fn sub(a: &u64, b: &u64) -> u64 {
        (((*a as u128 + MODULUS as u128) - *b as u128) % MODULUS as u128) as u64
    }

    fn neg(a: &u64) -> u64 {
        MODULUS - a
    }

    fn mul(a: &u64, b: &u64) -> u64 {
        ((*a as u128 * *b as u128) % MODULUS as u128) as u64
    }

    fn pow(a: &u64, b: &u64) -> u64 {
        ((*a as u128 ^ *b as u128) % MODULUS as u128) as u64
    }

    fn from_u64(x: u64) -> u64 {
        x % MODULUS
    }

    fn eq(a: &u64, b: &u64) -> bool {
        return *a == *b;
    }
}