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
}

pub trait Cipher {
    fn encrypt(&mut self, input: &[u8]) -> Vec<u8>;

    fn decrypt(&mut self, input: &[u8]) -> Vec<u8>;
}

pub trait AdvancedEncryptionStandard {
    fn key_schedule(key: [[u8; 4]; 4]) -> [[u8; 4]; 44];

    fn sub_bytes(&mut self, state: &mut [[u8; 4]; 4]);
    
    fn shift_rows(&mut self, state: &mut [[u8; 4]; 4]);
    
    fn mix_columns(&mut self, state: &mut [[u8; 4]; 4]);
    
    fn add_round_key(&mut self, state: &mut [[u8; 4]; 4], round: &mut usize);
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