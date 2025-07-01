// We have resources, things, it could be memory, file handlers, really whatever.
// RAII is simply the idea that some stack value owns this thing and when it goes out of scope,
// so does the resource
//
// Rust enforces it dont worry about it, just know that it exists
// Though if you want to drop something yourself, we can do that usind drop
// I dont know why you would want to but sure
fn main() {
    println!("Hello, world!");
}
