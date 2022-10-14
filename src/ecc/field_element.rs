use std::ops::{Add, Mul, Sub};

#[derive(Debug, PartialEq)]
pub struct FieldElement {
    num: i128,
    prime: i128,
}

impl FieldElement {
    pub fn new(num: i128, prime: i128) -> Self {
        assert!(
            num <= prime,
            "Num {} not in field range 0 to {}",
            num,
            prime - 1
        );
        FieldElement { num, prime }
    }

    pub fn pow(&self, exponent: u32) -> Self {
        let num = (i128::pow(self.num, exponent)) % self.prime;
        FieldElement::new(num, self.prime)
    }
}

impl Add for FieldElement {
    type Output = FieldElement;

    fn add(self, rhs: Self) -> Self::Output {
        assert!(self.prime == rhs.prime);
        let num = (self.num + rhs.num).rem_euclid(self.prime);
        FieldElement::new(num, self.prime)
    }
}

impl Sub for FieldElement {
    type Output = FieldElement;

    fn sub(self, rhs: Self) -> Self::Output {
        assert!(self.prime == rhs.prime);
        let num = (self.num - rhs.num).rem_euclid(self.prime);
        FieldElement::new(num, self.prime)
    }
}

impl Mul for FieldElement {
    type Output = FieldElement;

    fn mul(self, rhs: Self) -> Self::Output {
        assert!(self.prime == rhs.prime);
        let num = (self.num * rhs.num).rem_euclid(self.prime);
        FieldElement::new(num, self.prime)
    }
}

#[cfg(test)]
mod test {
    use super::FieldElement;

    #[test]
    fn test_field_element_add1() {
        let prime = 57;
        let lhs = FieldElement::new(44, prime);
        let rhs = FieldElement::new(33, prime);
        let ans = lhs + rhs;
        assert!(ans.num == 20);
    }

    #[test]
    fn test_field_element_add2() {
        let prime = 57;
        let one = FieldElement::new(17, prime);
        let two = FieldElement::new(42, prime);
        let three = FieldElement::new(49, prime);
        let ans = one + two + three;
        assert!(ans.num == 51);
    }

    #[test]
    fn test_field_element_sub1() {
        let prime = 57;
        let lhs = FieldElement::new(9, prime);
        let rhs = FieldElement::new(29, prime);
        let ans = lhs - rhs;
        assert!(ans.num == 37);
    }

    #[test]
    fn test_field_element_mul1() {
        let prime = 97;
        let one = FieldElement::new(95, prime);
        let two = FieldElement::new(45, prime);
        let three = FieldElement::new(31, prime);
        let ans = one * two * three;
        assert!(ans.num == 23);
    }

    #[test]
    fn test_field_element_pow1() {
        let prime = 13;
        let a = FieldElement::new(3, prime);
        let b = FieldElement::new(1, prime);
        assert!(a.pow(3) == b);
    }
}
