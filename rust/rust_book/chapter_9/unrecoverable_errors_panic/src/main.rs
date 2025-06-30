// We begin a chapter on errors and how we can approach handling them
// There are largely 2 types or errors:
//
// 1- Errors we can recover from, e.g, a file not creating
// 2- Errors we can't (shouldn't really) recover from, indexing on an element that is out of bounds
//
// We will begin talking about unrecoverable errors, which are the easiest to talk about
// unrecoverable errors, call panic!, which gives a message and beings exiting the program
// We can call a panic explicitly ourselves.
fn main() {
    // panic!("Crash and burn"); // panic will crash the program and exit

    let v = vec![1, 2, 3];
    //v[99]; //also panics since its out of bounds
}
