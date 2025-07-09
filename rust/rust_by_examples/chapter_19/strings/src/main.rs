// Strings a re just a vector of bytes with some extra functions and that are guaranteed to be
// valid UTF-8 char sequence.
// &str is a slice that points to a valid UTF-8 Sequence.
fn main() {
    // the to_string() converts string literals, &str, to String.
    let str: String = "Hi, my name is".to_string();
    // the most confusing part of strings in rust is the difference between chars and bytes
    // Strings are kinda complicated and rust shows that, characters are actually different byte
    // sizes and depending on whether you want to iterate using chars or bytes you will subtly
    // change the code. For example:
    for char in str.chars() {
        println!("The char is: {char}");
    }
    for byte in str.bytes() {
        println!("The byte is: {byte}");
    }

    // we can create a slice from a string
    let str_slice: &str = &str[..]; // this is a whole string slice
    println!("The string slice is: {str_slice}");
    let str_slice: &str = &str[1..]; // this is a whole string slice, but the first char
    println!("The string slice is: {str_slice}");
    let str_slice: &str = &str[..10]; // this is the first 10 chars
    println!("The string slice is: {str_slice}");
}
