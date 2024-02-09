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
    fn sub_bytes(&mut self, state: &mut [[u8; 4]; 4]);
    
    fn shift_rows(&mut self, state: &mut [[u8; 4]; 4]);
    
    fn mix_columns(&mut self, state: &mut [[u8; 4]; 4]);
    
    fn add_round_key(&mut self);
}