// A DSL is a mini language inside Rust macros.
// For example:

macro_rules! calculate {
    (eval $e:expr) => {
        let val: usize = $e; // makes sure the expr e expands to a unsigned integer
        println!("{} = {}", stringify!($e), val);
    };
}
fn main() {
    // We semi created a new keyword eval
    calculate! {eval 1 + 2};
    calculate! (eval 1 + 2);
}
