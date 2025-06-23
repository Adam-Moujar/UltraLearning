// Mut binding, we've talked about it extensively.
fn main() {
    let immutable = 1;
    let mut mutable = 1;

    // immutable = 2; // Doesn't work
    mutable = 2; // works
}
