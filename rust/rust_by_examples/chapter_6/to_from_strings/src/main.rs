// Converting this from and to string is also quite simple
//
// Converting to strings there are 2 methods
// You can either just implement toString trait
// You can also implement the fmt::Display trait which provides toString automatically
// and also allows us to print the type as well.
//
// Converting from strings is also quite useful. Primitives are done automatically,
// To convert custom types you have to implement fromstr trait
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Circle {
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

impl FromStr for Circle {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().parse() {
            Ok(num) => Ok(Circle { radius: num }),
            Err(e) => Err(e),
        }
    }
}
fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    let radius = "        3 ";
    let circle: Circle = radius.parse().unwrap();
    println!("{:?}", circle);
}
