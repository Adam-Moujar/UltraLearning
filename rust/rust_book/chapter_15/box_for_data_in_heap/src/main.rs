// Smart pointers are, as you guessed, pointers that are smart
// They contain some metadata and functionality in the background that makes sure
// that, for example, you can have multiple owners and when tehy all go out of scope then the data
// will be dropped.
//
// One other difference is that references borrow data while in many cases smart pointers actually
// own the data they point to.
//
// The most used smart pointer is Box, Box<T>. They allow data to be stored in the heap, while a
// pointer sits in the stack.
//
// Theres another usage, recursive types.
// Not allowed in rust, which is why we wrap the recursive type in a box. Not recursive anymore.

// Example of box and recursive type
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
