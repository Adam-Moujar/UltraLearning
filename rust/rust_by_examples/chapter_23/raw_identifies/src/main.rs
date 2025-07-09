// This is a pretty niche topic simply to future proof rust and make sure it doesnt break backwards
// compatibility.
// Rust doesnt allow you to create functions/variable with rust keywords.
// But what if you create a function now, and in the future rust updates and your function name is
// a keyword now?
// We can use raw identifiers

fn main() {
    // foo::r#try() // imagine there was a crate called foo which had a function try
    // Try is now a keyword in rust so it would complain that it wanted a function but you gave it
    // a keyword, we use r# to tell rust that is the name of the function not the keyword
}
