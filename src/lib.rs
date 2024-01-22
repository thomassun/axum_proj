/// this is the struct to represent a point in 2-axis dimision
/// `let a = Point(1,2);`
/// `assert_eq!(a.0,1);`
/// `assert_eq!(a.1,2);`
///
pub struct Point(i32, i32);
impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point(x, y)
    }
    pub fn take_it<P: Pat>(&self, p: P) {
        p.pat("df");
    }
}

pub trait Pat {
    fn pat(&self, p: &str) -> Self;
}

impl Pat for &str {
    fn pat(&self, p: &str) -> Self {
        self
    }
}
impl Pat for i32 {
    fn pat(&self, p: &str) -> Self {
        todo!()
    }
}
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

trait Supertrait
where
    Self: Sized,
{
    fn method<'a>(self) -> &'a str {
        "supertrait"
    }
}

trait Subtrait: Supertrait {
    // this looks like it might impl or
    // override Supertrait::method but it
    // does not
    fn method<'a>(self) -> &'a str {
        "subtrait"
    }
}

struct SomeType;

// adds Supertrait::method to SomeType
impl Supertrait for SomeType {}

// adds Subtrait::method to SomeType
impl Subtrait for SomeType {}

// both methods exist on SomeType simultaneously
// neither overriding or shadowing the other

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traits() {
        {
            assert_eq!(Subtrait::method(SomeType), "subtrait");
            assert_eq!(Supertrait::method(SomeType), "supertrait");
        };
    }
    #[test]
    fn test_name() {
        let p: Point = (32, 4).into();
        // p.take_it("df");
        // p.take_it(333);
        assert_eq!(p.1, 4);
        let p: Point = 32.into();
        assert_eq!(p.1, 32);
    }
    #[test]
    fn test_split() {
        let text = "apple>>banana>>cherry";
        let fruits: Vec<&str> = text.split(">>").collect();
        println!("{:?}", fruits); // Output: ["apple", "banana", "cherry"]
    }
}
