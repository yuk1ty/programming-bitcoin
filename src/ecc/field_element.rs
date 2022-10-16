use std::ops::{Add, Div, Mul, Sub};

use rug::{
    ops::{Pow, RemRounding},
    Integer,
};

#[derive(Debug, PartialEq)]
pub struct FieldElement {
    num: Integer,
    prime: Integer,
}

impl FieldElement {
    pub fn new(num: Integer, prime: Integer) -> Self {
        assert!(
            num <= prime,
            "Num {} not in field range 0 to {}",
            num,
            prime - 1
        );
        FieldElement { num, prime }
    }

    pub fn pow(&self, exponent: u32) -> Self {
        let num = self.num.clone().pow(exponent).rem_euc(&self.prime);
        FieldElement::new(num, self.prime.clone())
    }
}

impl Add for FieldElement {
    type Output = FieldElement;

    fn add(self, rhs: Self) -> Self::Output {
        assert!(self.prime == rhs.prime);
        let num = (self.num + rhs.num).rem_euc(&self.prime);
        FieldElement::new(num, self.prime)
    }
}

impl Sub for FieldElement {
    type Output = FieldElement;

    fn sub(self, rhs: Self) -> Self::Output {
        assert!(self.prime == rhs.prime);
        let num = (self.num - rhs.num).rem_euc(&self.prime);
        FieldElement::new(num, self.prime)
    }
}

impl Mul for FieldElement {
    type Output = FieldElement;

    fn mul(self, rhs: Self) -> Self::Output {
        assert!(self.prime == rhs.prime);
        let num = (self.num * rhs.num).rem_euc(&self.prime);
        FieldElement::new(num, self.prime)
    }
}

impl Div for FieldElement {
    type Output = FieldElement;

    fn div(self, rhs: Self) -> Self::Output {
        assert!(self.prime == rhs.prime);
        let num = (self.num
            * rhs
                .num
                .pow_mod(&(self.prime.clone() - Integer::from(2)), &self.prime)
                .unwrap())
        .rem_euc(&self.prime);
        FieldElement::new(num, self.prime)
    }
}

#[cfg(test)]
mod test {
    use super::FieldElement;
    use rug::Integer;

    impl FieldElement {
        pub fn new_i(num: i32, prime: i32) -> Self {
            Self {
                num: Integer::from(num),
                prime: Integer::from(prime),
            }
        }
    }

    #[test]
    fn test_field_element_add1() {
        let prime = 57;
        let lhs = FieldElement::new_i(44, prime);
        let rhs = FieldElement::new_i(33, prime);
        let ans = lhs + rhs;
        assert!(ans.num == 20);
    }

    #[test]
    fn test_field_element_add2() {
        let prime = 57;
        let one = FieldElement::new_i(17, prime);
        let two = FieldElement::new_i(42, prime);
        let three = FieldElement::new_i(49, prime);
        let ans = one + two + three;
        assert!(ans.num == 51);
    }

    #[test]
    fn test_field_element_sub1() {
        let prime = 57;
        let lhs = FieldElement::new_i(9, prime);
        let rhs = FieldElement::new_i(29, prime);
        let ans = lhs - rhs;
        assert!(ans.num == 37);
    }

    #[test]
    fn test_field_element_mul1() {
        let prime = 97;
        let one = FieldElement::new_i(95, prime);
        let two = FieldElement::new_i(45, prime);
        let three = FieldElement::new_i(31, prime);
        let ans = one * two * three;
        assert!(ans.num == 23);
    }

    #[test]
    fn test_field_element_div1() {
        let prime = 19;
        let one = FieldElement::new_i(2, prime);
        let two = FieldElement::new_i(7, prime);
        let ans = one / two;
        assert_eq!(ans.num, 3);
    }

    #[test]
    fn test_field_element_pow1() {
        let prime = 13;
        let a = FieldElement::new_i(3, prime);
        let b = FieldElement::new_i(1, prime);
        assert!(a.pow(3) == b);
    }
}
