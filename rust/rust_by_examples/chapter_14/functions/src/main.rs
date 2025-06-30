// Generics main function is to generalise functions and structs etc... which work on many types
// in just one definition.

// This is a generic function
// It takes a generic type, T as an arg.
// The generic type needs to be defined in <>.
fn foo<T>(arg: T) {}

// A struct can also be generic
// Takes a generic value as a tuple
struct SGen<T>(T);

// An implementation of a function for a generic type must be generic also
impl<T> SGen<T> {
    fn print() {
        println!("This is a generic function in SGEN");
    }
}

fn main() {}
