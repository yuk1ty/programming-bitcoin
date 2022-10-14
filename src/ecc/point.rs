#[derive(PartialEq)]
pub struct Point {
    a: i128,
    b: i128,
    x: i128,
    y: i128,
}

impl Point {
    pub fn new(x: i128, y: i128, a: i128, b: i128) -> Point {
        assert_eq!(y.pow(2), x.pow(3) + a * x + b);
        Point { a, b, x, y }
    }
}

#[cfg(test)]
mod test {
    use super::Point;

    #[test]
    fn test_point_creation() {
        let point = Point::new(-1, -1, 5, 7);
        assert!(point.x == -1);
        assert!(point.y == -1);
        assert!(point.a == 5);
        assert!(point.b == 7);
    }

    #[test]
    #[should_panic]
    fn test_fail_point_creation() {
        let _ = Point::new(-1, -2, 5, 7);
    }
}
