// We got 3 different types of structs

// tuple structs, essentially named tuples
struct Pair(i32, f32);

//A normal struct, classic c struct
struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

// Unit struct
struct Unit;
fn main() {
    println!("Hello, world!");
}
