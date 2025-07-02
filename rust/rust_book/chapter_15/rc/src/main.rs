// Rc is used when sharing heap data is important
//
// First, why cant we just use box?
//enum List {
//    Cons(i32, Box<List>),
//    Nil,
//}
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

//use crate::List::{Cons, Nil};
use crate::List::{Cons, Nil};
use std::rc::Rc;
fn main() {
    //let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    //let b = Cons(3, Box::new(a));
    //let c = Cons(4, Box::new(a));
    // This is why we cant use box. We moved a to the list in b
    // We cant re-use a.
    //
    // We could use references, but then we would need to litter our programs with lifetime,
    // and we would need to know how long our list lives which isnt always obvious.
    //
    // Now lets use Rc instead of Box

    fn main() {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        let b = Cons(3, Rc::clone(&a)); // Not a conventional clone, rc::clone is used to update
        // reference count
        let c = Cons(4, Rc::clone(&a));
    }

    // Example of rc count
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
