// Now generic type parameters are cool, but if they accept any type, we run into a problem
// There isn't much we can do with any type, since generic functions need to be able to work with
// them, the functionality we are allowed to use is, lacking to say the least.
//
// Trait bounds are ways to stipulate that the generic type must implement some functionality

// This says that the generic type T must implement the trait  std::fmt::Display
fn print<T: std::fmt::Display>(to_print: &T) {
    println!("{}", to_print);
}
fn main() {
    print(&4);
    print(&3);
    print(&2.0);
    print(&"Hi");

    // Look below for explanation
    let circle = Circle { radius: 1.2 };
    let rectangle = Rectangle {
        width: 1.0,
        height: 2.0,
    };
    let random = Random;
    area_of_shape(&circle);
    area_of_shape(&rectangle);
    //area_of_shape(&random); Doesnt implement Area trait so doesnt work

    // Again look below it will make sense
    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_prints(&string);
    //compare_prints(&array) // Doesnt use display, does use debug so wont work

    compare_types(&array, &vec);
}

// We can define our own structs and traits

// 2 structs which will implement our area trait
struct Rectangle {
    width: f32,
    height: f32,
}

struct Circle {
    radius: f32,
}

// a struct which wont implement area trait
struct Random;

trait Area {
    fn area(&self) -> f32;
}

impl Area for Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }
}

impl Area for Circle {
    fn area(&self) -> f32 {
        self.radius * self.radius * 3.141592
    }
}

fn area_of_shape<T: Area>(to_print: &T) {
    println!("The area of the shape is: {}", to_print.area());
}

// Empty bounds
struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

// We can have empty traits just to check for empty bounds

// Not only that we can have multiple bounds actually
use std::fmt::{Debug, Display};
fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: {:?}", t);
    println!("Display: {}", t);
}

// We can take 2 different types, T and U which both implement Debug
fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: {t:?}");
    println!("u: {u:?}");
}

// Now we can get pretty lengthy functions like
fn compare_print_types<T: Debug + Display, U: Debug + Display>(t: &T, u: &U) {}

// We can make them look better with the where clause
fn rewrite_print_types<T, U>(t: &T, u: &U)
where
    T: Display + Debug,
    U: Display + Debug,
{
}
