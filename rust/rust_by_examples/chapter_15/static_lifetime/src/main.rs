// Theres a special lifetime. 'static.
// It just means it lives for the rest of the program.
// The main usage of them is as string literal returns, &str,
// specifically when returning an error, since it makes sense
// likely the problem is not lasting any longer.
fn main() {
    println!("Hello, world!");
}
