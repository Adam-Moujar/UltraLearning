// Pretty standard programming language stuff
// We have 2 types of constant
// const and static

const THRESHOLD: i32 = 10;
static LANGUAGE: &str = "Rust";

// static is possiblity mutable, it has a static lifetime.
// Both have to be type explicit
fn main() {
    println!("Hello, world!");
}
