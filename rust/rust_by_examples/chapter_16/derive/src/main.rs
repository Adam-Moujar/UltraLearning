// A lot of traits can actually be automatically derived:
//
// Comparison traits: Eq, PartialEq, Ord, PartialOrd
// Clone, create T from &T
// Copy,
// Hash
// Default,
// Debug

// You derive like this
#[derive(Debug)]
struct Inches(i32);

fn main() {
    println!("Hello, world!");
}
