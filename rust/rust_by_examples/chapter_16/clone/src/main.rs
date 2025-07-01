// Largely we want to avoid copying data if at all possible, but sometimes it is either
// unavoidable, or actually more efficient since it allows us to implement a better algorithm.
//
// In those cases, clone is your go to, and we can even derive it
// (We largely cant copy, that for very inexpensive stuff like numbers and its done automatically by
// the rust compiler and we cant reimplement it, so we are really only left with clone, when
// talking about any serious struct type)
#[derive(Debug, Clone, Copy)]
// Simple enough to copy since its an empty struct
struct Unit;

// A tuple struct with resources that implements the `Clone` trait
// Cant implement copy here
#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);

fn main() {
    // Instantiate `Unit`
    let unit = Unit;
    // Copy `Unit`, there are no resources to move
    let copied_unit = unit;

    // Both `Unit`s can be used independently
    println!("original: {:?}", unit);
    println!("copy: {:?}", copied_unit);

    // Instantiate `Pair`
    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);

    // Move `pair` into `moved_pair`, moves resources
    let moved_pair = pair;
    println!("moved: {:?}", moved_pair);

    // Error! `pair` has lost its resources
    //println!("original: {:?}", pair);
    // TODO ^ Try uncommenting this line

    // Clone `moved_pair` into `cloned_pair` (resources are included)
    let cloned_pair = moved_pair.clone();
    // Drop the moved original pair using std::mem::drop
    drop(moved_pair);

    // Error! `moved_pair` has been dropped
    //println!("moved and dropped: {:?}", moved_pair);
    // TODO ^ Try uncommenting this line

    // The result from .clone() can still be used!
    println!("clone: {:?}", cloned_pair);
}
