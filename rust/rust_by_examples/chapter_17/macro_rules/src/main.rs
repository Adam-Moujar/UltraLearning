// I just covered macros in the rust book, but I do require some more work on it.
// Macros allow metaprogramming, writing rules that generate code for you

// Creating a simple macro called say_hello
macro_rules! say_hello {
    // () means the macro takes no arguments
    () => {
        // The macro will expand into the content of this block
        println!("Hello!");
    };
}

macro_rules! print_result {
    // Takes a expression and binds it to expression
    ($expression: expr) => {
        println!("{:?} = {:?}", stringify!($expression), $expression);
    };
}

// Creating another macro called create_function
macro_rules! create_function {
    // we take an argument of type ident and bind it func_name using $
    // ident is for function/variables names
    ($func_name:ident) => {
        fn $func_name() {
            // stringify converts ident into string
            println!("You called {:?}()", stringify!($func_name));
        }
    };
}
// very interestingly, we can actually overload a macro to take multiple different combinations of
// arguments
macro_rules! test {
    ($left:expr; and $right:expr) => {
        println!(
            "{:?} and {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left && $right
        )
    };
    ($left:expr; or $right:expr) => {
        println!(
            "{:?} or {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left || $right
        )
    };
}

// We can indicate that an argument may repeat atleast once, or * to show that it may repeat zero
// or more times
macro_rules! find_min {
    ($x: expr) => {
        $x
    };
    // This will match one or more expressions seperated by commas
    ($x: expr, $($y:expr),+) => {
        std::cmp::min($x, find_min!($($y),+))
        // we got recursive macros, its cooked
        // We have an expression, and a series of expressions, we call the find_min on the tail
        // expression list
    };
}
// These are the macro rules argument types, they are called designators
// block
// expr for expressions
// ident for function/variable names
// item
// literal is used for literal constants
// pat (pattern)
// path
// stmt statement
// tt token tree
// ty type
// vis visibility qualifier

create_function!(foo);
fn main() {
    say_hello!();
    foo();

    print_result!(1u32 + 1);

    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);

    println!("{}", find_min!(1));
    println!("{}", find_min!(1 + 2, 2));
    println!("{}", find_min!(5, 2 * 3, 4));
}
