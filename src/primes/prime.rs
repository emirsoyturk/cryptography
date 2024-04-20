use num_bigint::{BigInt, BigUint};
use num_traits::{One, Zero};

pub trait Prime {
    fn egcd(a: BigInt, b: BigInt) -> (BigInt, BigInt, BigInt);
    fn mod_inv(&self, other: &BigUint) -> Option<BigUint>;
    fn is_prime(&self, k: usize) -> bool;
    fn random_prime() -> BigUint;
}

impl Prime for BigUint {
    fn egcd(a: BigInt, b: BigInt) -> (BigInt, BigInt, BigInt) {
        if a.is_zero() {
            (b.clone(), BigInt::zero(), BigInt::one())
        } else {
            let (g, x, y) = Self::egcd(b.clone() % a.clone(), a.clone());
            (g, y - (b.clone() / a.clone()) * x.clone(), x.clone())
        }
    }

    fn mod_inv(&self, other: &BigUint) -> Option<BigUint> {
        let a = BigInt::from_biguint(num_bigint::Sign::Plus, self.clone());
        let m = BigInt::from_biguint(num_bigint::Sign::Plus, other.clone());
        let (g, x, _) = Self::egcd(a.clone(), m.clone());
        if g != BigInt::one() {
            None
        } else {
            Some(((x % &m + &m) % m).to_biguint().unwrap())
        }
    }

    fn is_prime(&self, k: usize) -> bool {
        if self % BigUint::from(2u64) == BigUint::from(0u64) {
            return self == &BigUint::from(2u64);
        }
        if self == &BigUint::from(1u64) {
            return false;
        }
        let mut d = self - BigUint::from(1u64);
        let mut r = 0;
        while d.clone() % BigUint::from(2u64) == BigUint::from(0u64) {
            d >>= 1;
            r += 1;
        }
        for _ in 0..k {
            let a = BigUint::from(2u64)
                + BigUint::from_bytes_be(&rand::random::<[u8; 32]>())
                    % (self - BigUint::from(4u64));
            let mut x = a.modpow(&d, self);
            if x == BigUint::from(1u64) || x == self - BigUint::from(1u64) {
                continue;
            }
            for _ in 0..r - 1 {
                x = x.modpow(&BigUint::from(2u64), self);
                if x == BigUint::from(1u64) {
                    return false;
                }
                if x == self - BigUint::from(1u64) {
                    break;
                }
            }
            if x != self - BigUint::from(1u64) {
                return false;
            }
        }

        true
    }

    fn random_prime() -> BigUint {
        loop {
            let p = BigUint::from_bytes_be(&rand::random::<[u8; 32]>());
            if Self::is_prime(&p, 100) {
                return p;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mod_inv() {
        assert_eq!(
            BigUint::from(5u64).mod_inv(&BigUint::from(12u64)),
            Some(BigUint::from(5u64))
        );
        assert_eq!(
            BigUint::from(17u64).mod_inv(&BigUint::from(12u64)),
            Some(BigUint::from(5u64))
        );
        assert_eq!(
            BigUint::from(103u64).mod_inv(&BigUint::from(12u64)),
            Some(BigUint::from(7u64))
        );
        assert_eq!(BigUint::from(32u64).mod_inv(&BigUint::from(4u64)), None);
    }
}
