use cryptography::Field;
use std::ops::{Add};

pub struct FieldElement<F: Field> {
    value: F::BaseType
}

impl<F> Add<&FieldElement<F>> for &FieldElement<F>
where
    F: Field,
{
    type Output = FieldElement<F>;

    fn add(self, rhs: &FieldElement<F>) -> Self::Output {
        Self::Output {
            value: <F as Field>::add(&self.value, &rhs.value),
        }
    }
}

impl<F> From<&F::BaseType> for FieldElement<F>
where
    F::BaseType: Clone,
    F: Field,
{
    fn from(value: &F::BaseType) -> Self {
        Self {
            value: value.clone(),
        }
    }
}

impl<F> From<u64> for FieldElement<F>
where
    F: Field,
{
    fn from(value: u64) -> Self {
        Self {
            value: F::from_u64(value),
        }
    }
}

impl<F> PartialEq<FieldElement<F>> for FieldElement<F>
where
    F: Field,
{
    fn eq(&self, other: &FieldElement<F>) -> bool {
        F::eq(&self.value, &other.value)
    }
}


#[cfg(test)]
mod tests {
    use crate::fields::u64_field::U64Field;
    use super::FieldElement;

    pub type U64TestField = U64Field<18446744069414584321>;

    #[test]
    fn test_add_two_field_element() {
        let a = FieldElement::<U64TestField>::from(10);
        let b = FieldElement::<U64TestField>::from(10);

        let c = &a + &b;

        assert!(c == FieldElement::<U64TestField>::from(20));
        assert!(c.value == 20);
    }
}