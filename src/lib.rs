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