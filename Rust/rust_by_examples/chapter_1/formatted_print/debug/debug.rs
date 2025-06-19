// To print using {}, a type must implement the fmt::Display trait
// which is only done automatically for the std library types.
//
// Instead we can use the fmt::Debug trait, all types can derive
// That is automatically create the implementation for the trait.

struct unPrintable(i32);

#[derive(Debug)]
struct DebugPrintable(i32);

// To debug print a structure which holds as a field another structure.
// the nested structure must also be debug printable.
#[derive(Debug)]
struct Deep(DebugPrintable);

fn main() {
    println!("Now {0:?} will print!", Deep(DebugPrintable(7)));
    println!("Now {0:#?} will print!", Deep(DebugPrintable(7)));
}

// the problem with debug printing is the lack of control over it
// It's only meant to print debug information
// You have the option of printing using {:#?} instead for pretty printing
