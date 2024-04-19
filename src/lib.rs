pub trait Field {
    type BaseType: Clone;

    // returns a + b
    fn add(a: &Self::BaseType, b: &Self::BaseType) -> Self::BaseType;

    // returns a - b
    fn sub(a: &Self::BaseType, b: &Self::BaseType) -> Self::BaseType;

    // returns a * b
    fn mul(a: &Self::BaseType, b: &Self::BaseType) -> Self::BaseType;

    // returns a ^ b
    fn pow(a: &Self::BaseType, b: &Self::BaseType) -> Self::BaseType;

    // returns -a
    fn neg(a: &Self::BaseType) -> Self::BaseType;

    // from u64
    fn from_u64(a: u64) -> Self::BaseType;

    // eq
    fn eq(a: &Self::BaseType, b: &Self::BaseType) -> bool;

    // is prime
    fn is_prime(a: &Self::BaseType) -> bool;

    // gcd
    fn gcd(a: Self::BaseType, b: Self::BaseType) -> Self::BaseType;

    // modular exponentiation
    fn exp(a: &Self::BaseType, b: &Self::BaseType, m: &Self::BaseType) -> Self::BaseType;

    // modular inverse
    fn inv(a: &Self::BaseType, m: &Self::BaseType) -> Option<Self::BaseType>;
}

pub trait Cipher {
    fn encrypt(&mut self, input: &[u8]) -> Vec<u8>;

    fn decrypt(&mut self, input: &[u8]) -> Vec<u8>;
}

pub trait AdvancedEncryptionStandard {
    fn key_schedule(key: [[u8; 4]; 4]) -> [[u8; 4]; 44];

    fn sub_bytes(&mut self, state: &mut [[u8; 4]; 4]);

    fn sub_bytes_inversed(&mut self, state: &mut [[u8; 4]; 4]);

    fn shift_rows(&mut self, state: &mut [[u8; 4]; 4]);

    fn shift_rows_inversed(&mut self, state: &mut [[u8; 4]; 4]);

    fn mix_columns(&mut self, state: &mut [[u8; 4]; 4]);

    fn mix_columns_inversed(&mut self, state: &mut [[u8; 4]; 4]);

    fn add_round_key(&mut self, state: &mut [[u8; 4]; 4], round: &mut usize, inversed: bool);
}

pub trait HashFunction {
    fn hash(&mut self, input: &[u8]) -> Vec<u8>;
}

pub fn galois_multiplication(a: u8, b: u8) -> u8 {
    let mut p = 0u8;
    let mut a = a;
    let mut b = b;

    for _ in 0..8 {
        if (b & 1) != 0 {
            p ^= a;
        }

        let bit_set: bool = (a & 0x80) != 0;

        a <<= 1;

        if bit_set {
            a ^= 0x1B;
        }

        b >>= 1;
    }

    p
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Number {
    Float32(f32),
    Float64(f64),
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Number::Float32(value) => write!(f, "{}", value),
            Number::Float64(value) => write!(f, "{}", value),
        }
    }
}

impl std::ops::Add for Number {
    type Output = Number;

    fn add(self, other: Number) -> Number {
        match self {
            Number::Float32(value) => match other {
                Number::Float32(other_value) => Number::Float32(value + other_value),
                Number::Float64(other_value) => Number::Float64(value as f64 + other_value),
            },
            Number::Float64(value) => match other {
                Number::Float32(other_value) => Number::Float64(value + other_value as f64),
                Number::Float64(other_value) => Number::Float64(value + other_value),
            },
        }
    }
}

impl std::ops::Sub for Number {
    type Output = Number;

    fn sub(self, other: Number) -> Number {
        match self {
            Number::Float32(value) => match other {
                Number::Float32(other_value) => Number::Float32(value - other_value),
                Number::Float64(other_value) => Number::Float64(value as f64 - other_value),
            },
            Number::Float64(value) => match other {
                Number::Float32(other_value) => Number::Float64(value - other_value as f64),
                Number::Float64(other_value) => Number::Float64(value - other_value),
            },
        }
    }
}

impl std::ops::Mul for Number {
    type Output = Number;

    fn mul(self, other: Number) -> Number {
        match self {
            Number::Float32(value) => match other {
                Number::Float32(other_value) => Number::Float32(value * other_value),
                Number::Float64(other_value) => Number::Float64(value as f64 * other_value),
            },
            Number::Float64(value) => match other {
                Number::Float32(other_value) => Number::Float64(value * other_value as f64),
                Number::Float64(other_value) => Number::Float64(value * other_value),
            },
        }
    }
}

impl std::ops::Div for Number {
    type Output = Number;

    fn div(self, other: Number) -> Number {
        match self {
            Number::Float32(value) => match other {
                Number::Float32(other_value) => Number::Float32(value / other_value),
                Number::Float64(other_value) => Number::Float64(value as f64 / other_value),
            },
            Number::Float64(value) => match other {
                Number::Float32(other_value) => Number::Float64(value / other_value as f64),
                Number::Float64(other_value) => Number::Float64(value / other_value),
            },
        }
    }
}

impl std::convert::From<f32> for Number {
    fn from(value: f32) -> Number {
        Number::Float32(value)
    }
}

impl std::convert::From<f64> for Number {
    fn from(value: f64) -> Number {
        Number::Float64(value)
    }
}

pub struct Point {
    pub x: Number,
    pub y: Number,
}
