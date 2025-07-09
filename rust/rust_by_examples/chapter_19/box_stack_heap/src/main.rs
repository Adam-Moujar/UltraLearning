// All values are stack allocated by default. If you want them to be heap allocated, you can
// use a Box<T>.

struct Point {
    x: f64,
    y: f64,
}

fn main() {
    // stack allocated point
    let stack_point = Point { x: 1.0, y: 2.0 };
    // heap allocated point
    let boxed_point: Box<Point> = Box::new(Point { x: 1.0, y: 2.0 });
}
