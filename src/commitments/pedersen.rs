use crate::primes::prime::Prime;
use cryptography::Commitment;
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use rand::Rng;

pub struct Pedersen {
    p: BigUint,
    q: BigUint,
    g: BigUint,
    h: BigUint,
}

impl Commitment for Pedersen {
    fn setup(&mut self, _security: u64) {
        self.p = BigUint::random_prime();
        self.q = self.p.clone() * BigUint::from(2u64) + BigUint::from(1u64);
        self.g = BigUint::from(rand::thread_rng().gen_range(0..self.q.to_u64().unwrap()));
        let num = rand::thread_rng().gen_range(0..self.q.to_u64().unwrap());
        self.h = self.g.modpow(&BigUint::from(num), &self.q);
    }

    fn setup_with_params(&mut self, p: BigUint, q: BigUint, g: BigUint, h: BigUint) {
        self.p = p;
        self.q = q;
        self.g = g;
        self.h = h;
    }

    fn commit(&mut self, input: &[u8]) -> Vec<u8> {
        vec![]
    }

    fn verify(&mut self, input: &[u8], commitment: &[u8]) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_pedersen() {
        let mut pedersen = Pedersen {
            p: BigUint::from_str("835633126610936170404251455173303609923742171077").unwrap(),
            q: BigUint::from_str("1671266253221872340808502910346607219847484342155").unwrap(),
            g: BigUint::from_str("853879156962728674489578584402624767640115848839").unwrap(),
            h: BigUint::from_str("312816528258481903392461143384659987330939968504").unwrap(),
        };
        pedersen.setup(256);
        let input = b"hello world";
        let commitment = pedersen.commit(input);
        assert!(pedersen.verify(input, &commitment));
    }
}
