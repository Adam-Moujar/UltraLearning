// Define a function called main.
// main is the entry point of every (not actually) rust program.
// The empty parenthesis means no parameters and there's no ->
// which means no return type aswell
//
// The function body is wrapped with {}
fn main() {
    println!("Hello, world!");

    // println! Is a rust macro, shown by the ! at the end of the keyword.
    // "Hello, world!" is passed in as an argument to println!
    // We end the statement with ;
}

// When running the program, we must first compile it.
// By running rustc program.rs or in the future just cargo run.
