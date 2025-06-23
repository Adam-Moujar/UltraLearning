// This is the first true foreign topic of Rust, ownership
// There are 3 broad rules we need to always remember:
// 1. Every value in Rust has an owner
// 2. There can only be one owner at a time
// 3. When the owner  goes out of scope, the value is dropped from memory

fn test_ownership(str: String) {
    ()
}

fn main() {
    {
        let s = "hello"; // s owns the value "hello"
    } // s falls out of scope here so the value is dropped, cant be used anymore
    // println!("{}, world!", s); // This doesn't compile since s got dropped

    let mut s = String::from("hello"); // these variables exist in heap, versus string literals
    s.push_str(", world!");
    println!("{s}");

    let s1 = String::from("Hello");
    let s2 = s1;
    // println!("{s1}, world!"); // This doesn't compile because s2 now owns the s1 value
    // And thus you can't use s1 since you can only have one owner.

    // instead, we can clone, so that they both own the value
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

    // trivial variables such as int, and floats and the primitives and very small
    // so they are automatically cloned

    let s3 = 1;
    let s4 = s3;

    println!("s3 = {s3}, s4 = {s4}");

    // Just like variables, functions can also get ownership
    // and so passing a variable to a function, passes ownership to the function.
    let s5 = String::from("hello");
    test_ownership(s5);
    // println!("The number is {s5}"); // this wont compile because the function owns s5, and then
    // destroyed.
}
