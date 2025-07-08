// Advanced functions
//
// Function pointers, we can pass not just closures as arguments, but also normal functions
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// there are differences, fn is a type rather than in the case of closures, which are impl trait
// like FnOnce, Fn, or FnMut.
// the fn pointer implements all three traits so you can pass a function anytime a closure is
// expected.
//
// Though there is a case where we only want to pass a function and that is using external
// code from a language which doesnt have closures, like C. C functions cannot accept closures
//
//
// Returning closures
// We return impl Trait syntax
// A quirky thing to keep inmind, closures have secret types assigned to them, so no 2 closures are
// the same type even if they implement the same Trait like impl Fn.
// This means if you ever wanted a vector of closures, tough luck
// Though you probably can get away with Box<dyn Fn> if they implement the same trait.
fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}
fn main() {
    println!("Hello, world!");
}
