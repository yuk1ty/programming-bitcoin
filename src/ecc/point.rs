use std::{fmt::Display, ops::Add};

use super::field_element::FieldElement;

#[derive(Clone, Debug, PartialEq)]
pub enum Points<T> {
    Point { x: T, y: T, a: T, b: T },
    Inf { a: T, b: T },
}

pub trait NewPoints<T> {
    fn new(x: T, y: T, a: T, b: T) -> Points<T>;
}

impl<T> Points<T> {
    pub fn inf(a: T, b: T) -> Points<T> {
        Points::Inf { a, b }
    }

    pub fn a(&self) -> &T {
        match &self {
            &Points::Point {
                x: _,
                y: _,
                a,
                b: _,
            } => a,
            &Points::Inf { a, b: _ } => a,
        }
    }

    pub fn b(&self) -> &T {
        match &self {
            Points::Point {
                x: _,
                y: _,
                a: _,
                b,
            } => b,
            Points::Inf { a: _, b } => b,
        }
    }
}

impl NewPoints<i128> for Points<i128> {
    fn new(x: i128, y: i128, a: i128, b: i128) -> Points<i128> {
        assert_eq!(y.pow(2), x.pow(3) + a * x + b);
        Points::Point { x, y, a, b }
    }
}

impl Add for Points<i128> {
    type Output = Points<i128>;

    fn add(self, rhs: Self) -> Self::Output {
        assert!(self.a() == rhs.a() && self.b() == rhs.b());
        match (&self, &rhs) {
            (
                &Points::Point {
                    x: x1,
                    y: y1,
                    a: a1,
                    b: b1,
                },
                &Points::Point { x: x2, y: y2, .. },
            ) => {
                if x1 == x2 && y1 != y2 {
                    return Points::Inf { a: a1, b: b1 };
                }

                if x1 != x2 {
                    let s = (y2 - y1) / (x2 - x1);
                    let x = s.pow(2) - x1 - x2;
                    let y = s * (x1 - x) - y1;
                    return Points::Point { x, y, a: a1, b: b1 };
                }

                if self == rhs && y1 == 0 {
                    return Points::Inf { a: a1, b: b1 };
                }

                if self == rhs {
                    let s = (3 * x1.pow(2) + a1) / (2 * y1);
                    let x = s.pow(2) - 2 * x1;
                    let y = s * (x1 - x) - y1;
                    return Points::Point { x, y, a: a1, b: b1 };
                }

                unreachable!("Theoretically unreachable point!")
            }
            (Points::Inf { .. }, Points::Point { .. }) => rhs,
            (Points::Point { .. }, Points::Inf { .. }) => self,
            (Points::Inf { .. }, Points::Inf { .. }) => {
                unreachable!("You can't pass both points at infinity here.")
            }
        }
    }
}

impl NewPoints<FieldElement> for Points<FieldElement> {
    fn new(
        x: FieldElement,
        y: FieldElement,
        a: FieldElement,
        b: FieldElement,
    ) -> Points<FieldElement> {
        assert_eq!(y.pow(2), x.pow(3) + a.clone() * x.clone() + b.clone());
        Points::Point { x, y, a, b }
    }
}

impl Add for Points<FieldElement> {
    type Output = Points<FieldElement>;

    fn add(self, rhs: Self) -> Self::Output {
        assert!(self.a() == rhs.a() && self.b() == rhs.b());
        match (self.clone(), rhs.clone()) {
            (
                Points::Point {
                    x: x1,
                    y: y1,
                    a: a1,
                    b: b1,
                },
                Points::Point { x: x2, y: y2, .. },
            ) => {
                if x1 == x2 && y1 != y2 {
                    return Points::Inf { a: a1, b: b1 };
                }

                if x1 != x2 {
                    let s = (y2 - y1.clone()) / (x2.clone() - x1.clone());
                    let x = s.pow(2) - x1.clone() - x2.clone();
                    let y = s * (x1 - x.clone()) - y1;
                    return Points::Point { x, y, a: a1, b: b1 };
                }

                if self == rhs && y1 == 0 {
                    return Points::Inf { a: a1, b: b1 };
                }

                if self == rhs {
                    let s = (3 * x1.pow(2) + a1.clone()) / (2 * y1.clone());
                    let x = s.pow(2) - 2 * x1.clone();
                    let y = s * (x1 - x.clone()) - y1;
                    return Points::Point { x, y, a: a1, b: b1 };
                }

                unreachable!("Theoretically unreachable point!")
            }
            (Points::Inf { .. }, Points::Point { .. }) => rhs,
            (Points::Point { .. }, Points::Inf { .. }) => self,
            (Points::Inf { .. }, Points::Inf { .. }) => {
                unreachable!("You can't pass both points at infinity here.")
            }
        }
    }
}

impl<T: Display> Display for Points<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Points::Point { x, y, a, b } => write!(f, "Point({}, {})_{}_{}", x, y, a, b),
            Points::Inf { .. } => write!(f, "Point(infinity)"),
        }
    }
}

#[cfg(test)]
mod test {
    use rug::Integer;

    use crate::ecc::{field_element::FieldElement, point::NewPoints};

    use super::Points;

    #[test]
    fn test_point_creation() {
        let point = Points::new(-1, -1, 5, 7);
        assert!(matches!(
            point,
            Points::Point {
                x: -1,
                y: -1,
                a: 5,
                b: 7
            }
        ));
    }

    #[test]
    fn test_inf_point_creation() {
        let point = Points::inf(5, 7);
        assert!(matches!(point, Points::Inf { a: 5, b: 7 }));
    }

    #[test]
    #[should_panic]
    fn test_fail_point_creation() {
        let _ = Points::new(-1, -2, 5, 7);
    }

    #[test]
    fn test_add1() {
        let a = Points::inf(5, 7);
        let b = Points::new(2, 5, 5, 7);
        assert!(matches!(
            a + b,
            Points::Point {
                x: 2,
                y: 5,
                a: 5,
                b: 7
            }
        ));
    }

    #[test]
    fn test_add2() {
        let a = Points::inf(5, 7);
        let b = Points::new(2, 5, 5, 7);
        assert!(matches!(
            b + a,
            Points::Point {
                x: 2,
                y: 5,
                a: 5,
                b: 7
            }
        ));
    }

    #[test]
    fn test_add3() {
        let a = Points::new(-1, -1, 5, 7);
        let b = Points::new(-1, 1, 5, 7);
        assert!(matches!(a + b, Points::Inf { a: 5, b: 7 }));
    }

    #[test]
    fn test_add4() {
        let a = Points::new(3, 7, 5, 7);
        let b = Points::new(-1, -1, 5, 7);
        assert!(matches!(
            a + b,
            Points::Point {
                x: 2,
                y: -5,
                a: 5,
                b: 7
            }
        ));
    }

    #[test]
    fn test_add5() {
        let a = Points::new(-1, -1, 5, 7);
        let b = Points::new(-1, -1, 5, 7);
        assert!(matches!(
            a + b,
            Points::Point {
                x: 18,
                y: 77,
                a: 5,
                b: 7
            }
        ));
    }

    fn field_elem(num: i32, prime: i32) -> FieldElement {
        FieldElement::new(Integer::from(num), Integer::from(prime))
    }

    #[test]
    fn test_field_elem_add1() {
        let prime = 223;
        let a = FieldElement::new(Integer::from(0), Integer::from(prime));
        let b = FieldElement::new(Integer::from(7), Integer::from(prime));
        let x1 = FieldElement::new(Integer::from(192), Integer::from(prime));
        let y1 = FieldElement::new(Integer::from(105), Integer::from(prime));
        let x2 = FieldElement::new(Integer::from(17), Integer::from(prime));
        let y2 = FieldElement::new(Integer::from(56), Integer::from(prime));
        let p1 = Points::new(x1, y1, a.clone(), b.clone());
        let p2 = Points::new(x2, y2, a.clone(), b.clone());
        assert_eq!(
            p1 + p2,
            Points::Point {
                x: field_elem(170, prime),
                y: field_elem(142, prime),
                a: field_elem(0, prime),
                b: field_elem(7, prime)
            }
        );
    }
}
