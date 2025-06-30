// Closures can capture variables in 3 different ways
// 1. reference &T
// 2. mutable reference &mut T
// 3. by value T
use std::mem;
fn main() {
    println!("Hello, world!");

    let color = String::from("green");

    // `println!` only requires arguments by immutable reference
    // So the closure will only immutable borrow color
    let print = || println!("`color`: {}", color);

    let _reborrow = &color;
    print(); // If print was a immutable borrow, you couldn't reborrow it

    // A move or reborrow is allowed after the final use of `print`
    let _color_moved = color;

    let mut count = 0;
    // A closure to increment `count` could take either `&mut count` or `count`
    // but `&mut count` is less restrictive so it takes that. Immediately
    // borrows `count`.
    //
    // A `mut` is required on `inc` because a `&mut` is stored inside. Thus,
    // calling the closure mutates `count` which requires a `mut`.
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // let _reborrow = &count; Cant do this because inc mutable borrows and you cant have mutable
    // borrows and any other borrow
    inc();

    // The closure no longer needs to borrow `&mut count`. Therefore, it is
    // possible to reborrow without an error
    let _count_reborrowed = &mut count;

    // A non-copy type.
    let movable = Box::new(3);

    // `mem::drop` requires `T` so this must take by value. A copy type
    // would copy into the closure leaving the original untouched.
    // A non-copy must move and so `movable` immediately moves into
    // the closure.
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };
    consume();
    // consume(); // Cant call it twice since consume, consumes the variable and cant be used again

    let haystack = vec![1, 2, 3];

    // You can use the move keyword to force closure to take ownership of the values, haystack in
    // this case
    let contains = move |needle| haystack.contains(needle);
}
