use cryptography::Field;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub struct FieldElement<F: Field> {
    value: F::BaseType
}

impl<F> Add<&FieldElement<F>> for &FieldElement<F>
where
    F: Field,
{
    type Output = FieldElement<F>;

    fn add(self, other: &FieldElement<F>) -> Self::Output {
        Self::Output {
            value: <F as Field>::add(&self.value, &other.value),
        }
    }
}

impl <F> Sub<&FieldElement<F>> for &FieldElement<F> 
where 
    F: Field
{
    type Output = FieldElement<F>;
    
    fn sub(self, other: &FieldElement<F>) -> Self::Output {
        Self::Output {
            value: <F as Field>::sub(&self.value, &other.value)
        }
    }
}

impl <F> Mul<&FieldElement<F>> for &FieldElement<F> 
where 
    F: Field
{
    type Output = FieldElement<F>;
    
    fn mul(self, other: &FieldElement<F>) -> Self::Output {
        Self::Output {
            value: <F as Field>::mul(&self.value, &other.value)
        }
    }
}

impl <F> Neg<> for &FieldElement<F> 
where 
    F: Field
{
    type Output = FieldElement<F>;
    
    fn neg(self) -> Self::Output {
        Self::Output {
            value: <F as Field>::neg(&self.value)
        }
    }
}

impl <F> AddAssign<&FieldElement<F>> for FieldElement<F> 
where 
    F: Field
{    
    fn add_assign(&mut self, other: &FieldElement<F>) {
        *self = FieldElement::<F> {
            value: (<F as Field>::add(&self.value, &other.value))
        }
    }
}

impl <F> SubAssign<&FieldElement<F>> for FieldElement<F> 
where 
    F: Field
{    
    fn sub_assign(&mut self, other: &FieldElement<F>) {
        *self = FieldElement::<F> {
            value: (<F as Field>::sub(&self.value, &other.value))
        }
    }
}


impl <F> MulAssign<&FieldElement<F>> for FieldElement<F> 
where 
    F: Field
{    
    fn mul_assign(&mut self, other: &FieldElement<F>) {
        *self = FieldElement::<F> {
            value: (<F as Field>::mul(&self.value, &other.value))
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
        let a: FieldElement<U64TestField> = FieldElement::<U64TestField>::from(10);
        let b: FieldElement<U64TestField> = FieldElement::<U64TestField>::from(10);

        let c: FieldElement<U64TestField> = &a + &b;

        assert!(c == FieldElement::<U64TestField>::from(20));
        assert!(c.value == 20);
    }

    #[test]
    fn test_add_assign_two_field_element() {
        let mut a: FieldElement<U64TestField> = FieldElement::<U64TestField>::from(10);
        let b: FieldElement<U64TestField> = FieldElement::<U64TestField>::from(10);

        a += &b;

        assert!(a == FieldElement::<U64TestField>::from(20));
        assert!(a.value == 20);
    }

    #[test]
    fn test_sub_two_field_element() {
        let a: FieldElement<U64TestField> = FieldElement::<U64TestField>::from(10);
        let b: FieldElement<U64TestField> = FieldElement::<U64TestField>::from(5);

        let c: FieldElement<U64TestField> = &a - &b;

        assert!(c == FieldElement::<U64TestField>::from(5));
        assert!(c.value == 5);
    }

    #[test]
    fn test_sub_assign_two_field_element() {
        let mut a: FieldElement<U64TestField> = FieldElement::<U64TestField>::from(10);
        let b: FieldElement<U64TestField> = FieldElement::<U64TestField>::from(5);

        a -= &b;        

        assert!(a == FieldElement::<U64TestField>::from(5));
        assert!(a.value == 5);
    }

    #[test]
    fn test_neg_field_element() {
        let a: FieldElement<U64TestField> = FieldElement::<U64TestField>::from(18446744069414584320);

        let c: FieldElement<U64TestField> = -&a;

        assert!(c == FieldElement::<U64TestField>::from(1));
        assert!(c.value == 1);
    }

    #[test]
    fn test_mul_field_element() {
        let a: FieldElement<U64TestField> = FieldElement::<U64TestField>::from(14);
        let b: FieldElement<U64TestField> = FieldElement::<U64TestField>::from(14);

        let c: FieldElement<U64TestField> = &a * &b;

        assert!(c == FieldElement::<U64TestField>::from(196));
        assert!(c.value == 196);
    }

    #[test]
    fn test_mul_assign_field_element() {
        let mut a: FieldElement<U64TestField> = FieldElement::<U64TestField>::from(14);
        let b: FieldElement<U64TestField> = FieldElement::<U64TestField>::from(14);

        a *= &b;

        assert!(a == FieldElement::<U64TestField>::from(196));
        assert!(a.value == 196);
    }
}