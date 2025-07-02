// We are in the potential unsafe side of rust!
// This is a big milestone
// We are doing interior mutability, or in other words, mutating data which has immutable
// references to it. A big no-no in rust usually
//
// Box T, like normal rust, enforces borrow checker rules at compile time
// RefCell T doesnt, like non rust code, it enforces it during runtime, and if you break the rules,
// we get a panic!
//
// The whole idea behind refcell is that, a box T where borrow check is used at runtime rather than
// compile time. The idea being simple, sometimes we know better than the compiler.
//
// Rc and RefCell both cant be used during multi-threaded contexts. I didnt note it in the Rc file,
// but its important that I remember this
use std::cell::RefCell;
fn main() {
    let x = RefCell::new(5);

    //let y = &mut x; // Cannot do this since x is immutable so cannot borrow mutably
    x.replace(6);

    println!("X is currently: {}", x.borrow());

    // Since it happens at runtime, what do you think will happen if we mutably borrow twice?
    // Since rust only allows one mutable reference?
    //
    // It panics
    let a = x.borrow_mut();
    let b = x.borrow_mut();
}
