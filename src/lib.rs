/// this is the struct to represent a point in 2-axis dimision
/// `let a = Point(1,2);`
/// `assert_eq!(a.0,1);`
/// `assert_eq!(a.1,2);`
///
pub struct Point(i32, i32);

impl From<(i32, i32)> for Point {
    fn from(value: (i32, i32)) -> Self {
        Point(value.0, value.1)
    }
}
impl From<i32> for Point {
    fn from(v: i32) -> Self {
        Point(v, v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let p: Point = (32, 4).into();
        assert_eq!(p.1, 4);
        let p: Point = 32.into();
        assert_eq!(p.1, 32);
        let x = i64::from(3i32);
    }
}
