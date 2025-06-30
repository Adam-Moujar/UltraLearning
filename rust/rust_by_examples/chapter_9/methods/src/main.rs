// Methods & associated functions
// Associated functions are functions in a impl block of the type
// Methods are special associated function where one of the paramters is self or &self
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    // Associated function
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Associated function
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}
struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // Method & Associated function
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        ((x1 - x2) * (y1 - y2)).abs()
    }

    // Method & Associated function
    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // Method & Associated function
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}
fn main() {
    println!("Hello, world!");
}
