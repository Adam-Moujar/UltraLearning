// You should minimize the amount of unsafe code
// Unsafe is a keyword that lets us do things that Rust otherwise stops us from doing:
// The things we are allowed to do are:
// - Dereferencing raw pointers
// - Calling functions/methods that are unsafe
// - Accessing or modifying static mutable variables
// - Implementing unsafe traits
//
// Lets briefly look at raw pointers and unsafe functions
use std::slice;
fn main() {
    let raw_p: *const u32 = &10;

    unsafe {
        // The idea of a raw pointer is that its well, a raw pointer
        // And if the value is moved, the pointer does not update
        // So if we derefence it, there's no guarantee that the memory address is valid
        // Which is why we need an unsafe block to use it
        assert!(*raw_p == 10);
    }

    let some_vector = vec![1, 2, 3, 4];

    let pointer = some_vector.as_ptr();
    let length = some_vector.len();

    unsafe {
        // from_raw_parts is a function that returns a slide from a pointer to a vector and a
        // length. Again its unsafe since it uses raw pointers and makes no promises whether the
        // slice it gets is bigger than memory we are allowed to access
        let my_slice: &[u32] = slice::from_raw_parts(pointer, length);

        assert_eq!(some_vector.as_slice(), my_slice);
    }
}
