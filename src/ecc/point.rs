use std::ops::Add;

#[derive(Debug, PartialEq)]
pub struct Point {
    a: i128,
    b: i128,
    x: Option<i128>,
    y: Option<i128>,
}

impl Point {
    pub fn new(x: Option<i128>, y: Option<i128>, a: i128, b: i128) -> Point {
        if let (Some(x), Some(y)) = (x, y) {
            assert_eq!(y.pow(2), x.pow(3) + a * x + b);
        }
        Point { a, b, x, y }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        if self.x.is_none() {
            return rhs;
        }
        if rhs.x.is_none() {
            return self;
        }

        // TODO add more conditions later
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::Point;

    #[test]
    fn test_point_creation() {
        let point = Point::new(Some(-1), Some(-1), 5, 7);
        assert!(point.x.unwrap() == -1);
        assert!(point.y.unwrap() == -1);
        assert!(point.a == 5);
        assert!(point.b == 7);
    }

    #[test]
    fn test_inf_point_creation() {
        let point = Point::new(None, None, 5, 7);
        assert!(point.a == 5);
        assert!(point.b == 7);
    }

    #[test]
    #[should_panic]
    fn test_fail_point_creation() {
        let _ = Point::new(Some(-1), Some(-2), 5, 7);
    }

    #[test]
    fn test_add1() {
        let a = Point::new(None, None, 5, 7);
        let b = Point::new(Some(2), Some(5), 5, 7);
        assert_eq!(a + b, Point::new(Some(2), Some(5), 5, 7));
    }

    #[test]
    fn test_add2() {
        let a = Point::new(None, None, 5, 7);
        let b = Point::new(Some(2), Some(5), 5, 7);
        assert_eq!(b + a, Point::new(Some(2), Some(5), 5, 7));
    }
}
