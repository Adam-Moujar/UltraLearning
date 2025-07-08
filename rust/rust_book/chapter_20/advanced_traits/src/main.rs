// We can get into the nitty gritty of traits
// Associated types
// They connect a type placeholder with a trait
// and now we can use this placeholder in method definitions
// Later when we implement the trait, we can define what the placeholder was
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    counter: u32,
}

impl Iterator for Counter {
    type Item = u32; // here we give a concrete type for the placeholder

    fn next(&mut self) -> Option<Self::Item> {
        self.counter += 1;
        Some(self.counter)
    }
}

// theres a question about why not use generics?
// The problem with generics is that we could implement iterator for multiple types
// whether string or whatelse. Then there is some ambiguity about what iterator and what next to
// use and we would have to type annotate our variables to clear this ambiguity.
//
// Here we only implement interator once for counter
// It also becomes part of the trait contract. Not only do you have to implement the functions
// you must also provide a standin for the associated type placeholder
//
// Default generic types and operator overloading
// We can specify a default generic type so we dont need to specify it everytime
// for example:

use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    y: i32,
    x: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// The add trait is defined like so
// trait Add<Rhs=Self>{
//      type Output;
//
//      fn add(self, rhs:Rhs) -> Self::Output;
// }

// As you can see it is generic, given by the angular brackets
// It takes a generic Rhs but it's also given a default, which is the
// same type of the object it implements it
//
// we could implement add not on self for example:

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// Disambiguating between methods of same name
// theres nothing stopping having 2 traits with a same name required function
// and then implementing both traits on a type
// in that case when we call the method, which of the 2 functions will run?
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}
// Another interesting problem
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// super traits
// sometimes we need traits that depend on other traits
// in the cases where to implement a trait, we first need to implement a second trait
use std::fmt;

// we define outlineprint which depends on std::fmt::Display
trait OutlinePrint: fmt::Display {
    fn outlint_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
struct NewPoint {
    x: i32,
    y: i32,
}

impl OutlinePrint for NewPoint {} // we need to also implement display, remember

impl fmt::Display for NewPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Using NewType pattern to implement external traits on external types
// we can only implement traits on a type if either the trait, or the type are local to our crate.
// If neither are local, then usually it would be tough luck, but using the NewType Pattern, we can
// actually implement External traits on External Types, its a bit of a hacky method
//
// We essentially create a empty wrapper around the type we want to use

// in this case, vec string is the type we want to implement the trait on
struct Wrapper(Vec<String>);

// and the trait we want to implement is std::fmt::Display
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
// There is a downside, since the vec is wrapped in a new type, we dont have access
// to the functions of the inner type, except if we impl deref on wrapper, in which case, then we
// would have access to the function of vector on wrapper
fn main() {
    let person = Human;
    person.fly(); // This calls the human implementation
    Pilot::fly(&person); // This is for the pilot fly implementation
    Wizard::fly(&person); // This is for the wizard impleentation

    // println!("A baby dog is called a {}", Animal::baby_name());  // This cant be done since its
    // a trait and many types can implement this trait. So the compiler doesnt know which of those
    // functions to run, we would need to use
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    // or
    println!("A baby dog is called a {}", Dog::baby_name());

    let p = NewPoint { x: 1, y: 2 };
    p.outlint_print();

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");
}
