// We are here, final chapter before the project
// Well mostly final chapter before moving onto rust by examples but
// WE did it.
// In this chapter we will cover all the advanced rust features we can use
// Starting with unsafe rust:

static mut COUNTER: u32 = 0;

fn main() {
    // boom unsafe block
    // go crazy, if it breaks its on you
    // Here are the five things we can do that we usually cant
    // derefence a raw pointer
    // call an unsafe function or method
    // access or modify a mutable static variable
    // implement unsafe trait
    // access fields of a union
    unsafe {
        // Raw pointers
        // New special type of pointers which are
        // *const T
        // *mut T
        let mut num = 5;

        // Technically this is safe, having raw pointers by themselves
        // is ok, we just cant dereference them in safe rust.
        let r1 = &raw const num;
        let r2 = &raw mut num;

        // Now this is unsafe
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);

        // Another thing you might have noticed is that we just created a mutable and immutable
        // reference to a variable when usually we can't.
        // Thats another perk of unsafe rust

        dangerous(); // This is okay

        // We can make safe abstraction over unsafe code
        // we can use slice.split_at_mut as an example of this
        let mut v = vec![1, 2, 3, 4, 5, 6];
        let r = &mut v[..]; // a mutable slice of the whole vector

        let (a, b) = r.split_at_mut(3);

        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);

        //lets see how it could be implemented below

        // Modifying a mutable static variable
        // SAFETY: Calling this from more than a single thread is undefined
        // since 2 threads modifying 1 single value at the same time is a huuuge problem with many
        // different possibilities. Make sure you only do this if its single-threaded

        // When writing a unsafe function it is convention to make a SAFETY doc comment which
        // explains what the caller needs to do to make sure to use the function safely.
        // It is also good convention in unsafe operations to write a normal SAFETY comment writing
        // how the safety rules are upheld

        COUNTER += 1;

        // Accessing Fields of a Union

        // The final action that works only with unsafe is accessing fields of a union. A union is
        // similar to a struct, but only one declared field is used in a particular instance at
        // one time. Unions are primarily used to interface with unions in C code
    }
    //dangerous(); // This doesnt work since we are calling an unsafe function outside of the unsafe
    // block
    //
    // We can also work with external code, code written in other languages.
    // Rust has the keyword extern which allows the creation of a Foreign Function Interface (FFI)
    // FFI is a way for prgoramming langauges to define functions and allow different programming
    // language to call those functions
    // See below for more
    print_abs_of_minus_three();
}

// unsafe functions are created with the unsafe keyword aswell
unsafe fn dangerous() {}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr(); // this gives us the raw ptr of a slice

    assert!(mid <= len); // makes sure the mid you gave it isnt larger than the slice

    // (&mut values[..mid], &mut values[mid..])
    // This does not work because we are borrowing a reference as mutable twice
    // Even though this is totally okay since we are borrowing different parts of the slice
    // But rust does not know better and thinks this is wrong
    // and so we need to unsafe rust
    use std::slice;
    unsafe {
        (
            // from_raw_parts_mut and ptr.add are both unsafe functions
            // we know that the memory is safe since it is part of the slice
            // but the function trusts that we know what we are doing
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }

    // This function is safe even though its got an unsafe block
}

// Extern section
// We can set up and use the abs function from the C standard library
// Here we use the C application binary interface  (ABI) to call the c standard library abs
// Rust has a list of ABIs they support
//
// Every item declared within unsafe extern is implicitly unsafe, but it doesn't mean the
// actual function is unsafe. The abs function in this example has no unsafe memory considerations
// and it will work with i32s as well, so there's no problem that could arise in this case.
// We can note these functions with the special keyword safe
unsafe extern "C" {
    // safe function which means we dont need unsafe blocks
    // It doesnt mean its safe, we promise Rust its safe,
    // whether or not it is actually safe is up to us.
    safe fn abs(input: i32) -> i32;
}

fn print_abs_of_minus_three() {
    // unnecessary unsafe block since abs is denoted safe
    unsafe {
        println!("The absolute value of -3 according to C: {}", abs(-3));
    }
}
