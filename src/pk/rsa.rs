use num_bigint::BigUint;
use num_traits::ToPrimitive;

use crate::primes::prime::Prime;

pub struct RSA {
    e: BigUint,
    d: BigUint,
    n: BigUint,
}

impl RSA {
    pub fn new() -> Result<Self, &'static str> {
        let p = BigUint::random_prime();
        let q = BigUint::random_prime();

        let n: BigUint = &p * &q;
        let fi = (&p - BigUint::from(1u64)) * (&q - BigUint::from(1u64));
        let e = BigUint::random_prime();
        let d = match e.mod_inv(&fi) {
            Some(d) => d,
            None => return Err("e has no modular inverse"),
        };

        Ok(Self { e, d, n })
    }

    pub fn encrypt(&self, m: &BigUint) -> BigUint {
        m.modpow(&self.e, &self.n)
    }

    pub fn decrypt(&self, c: &BigUint) -> BigUint {
        c.modpow(&self.d, &self.n)
    }

    pub fn encrypt_str(&self, s: &str) -> Vec<BigUint> {
        s.chars()
            .map(|c| self.encrypt(&BigUint::from(c as u64)))
            .collect()
    }

    pub fn decrypt_str(&self, v: &[BigUint]) -> String {
        v.iter()
            .map(|c| char::from(c.modpow(&self.d, &self.n).to_u32().unwrap() as u8))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rsa() {
        let rsa = RSA::new().unwrap();

        let m = BigUint::from(23u64);

        let c = rsa.encrypt(&m);

        let m2 = rsa.decrypt(&c);
        assert_eq!(m, m2);
    }

    #[test]
    fn test_rsa_str() {
        let rsa = RSA::new().unwrap();

        let s = "Hello, World!";
        let v = rsa.encrypt_str(s);
        let s2 = rsa.decrypt_str(&v);
        assert_eq!(s, s2);
    }
}
