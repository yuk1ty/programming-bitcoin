use std::{fmt::Display, ops::Add};

#[derive(Debug, PartialEq)]
pub enum Points<T> {
    Point { x: T, y: T, a: T, b: T },
    Inf { a: T, b: T },
}

impl Points<i128> {
    pub fn new(x: i128, y: i128, a: i128, b: i128) -> Points<i128> {
        assert_eq!(y.pow(2), x.pow(3) + a * x + b);
        Points::Point { x, y, a, b }
    }

    pub fn inf(a: i128, b: i128) -> Points<i128> {
        Points::Inf { a, b }
    }

    pub fn a(&self) -> i128 {
        match *self {
            Points::Point {
                x: _,
                y: _,
                a,
                b: _,
            } => a,
            Points::Inf { a, b: _ } => a,
        }
    }

    pub fn b(&self) -> i128 {
        match *self {
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

impl Add for Points<i128> {
    type Output = Points<i128>;

    fn add(self, rhs: Self) -> Self::Output {
        assert!(self.a() == rhs.a() && self.b() == rhs.b());
        // TODO この実装にする場合、無限遠点が含まれる計算の捌き方がこれでよかったかを検討する必要がありそう。
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
            // TODO 両方とも無限遠点だった場合はそもそも何かがおかしい？
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
}
