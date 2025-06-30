// Observe how using a closure as a function parameter requires generics,
// which is necessary because of how they are defined:
// `F` must be generic.
fn apply<F>(f: F)
where
    F: FnOnce(),
{
    f();
}
fn main() {
    println!("Hello, world!");
}
