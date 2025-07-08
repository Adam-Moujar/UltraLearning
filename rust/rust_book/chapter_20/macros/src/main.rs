// Macros, we've been using them, println!.
// Lets see how they work and how to make our own.
//
// Macros are ways to write code, that writes code, its a type of metaprogramming.
// derive!, which is a macro we covered, writes trait implementations for you, for example.
//
// Macros get expanded before compile time. One thing unique about macros is that you
// need to bring them into scope before you use them as opposed to functions which
// can be called from anywhere as long as the function is in scope.
//
// Declaring macros with macro_rules!
// The most widely used macro is the declarative macro.
// they are sometimes refered to as macro by examples, kind of a cool name, is like match
// statements except meta version.
// It matches a source code and depending on the different patterns we can have, it will
// generate different source code. Let's look at the vec! example, its defined like:
// (Not quite the same, we are skipping optimisation code)
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
// The macro_export is to make sure the macro is brought into scope when the crate is also brought
// into scope, otherwise there is no way the macro can be used.
// We write macro_rules! and the name of the macro, vec in this case.
// Again, macro rules are kinda like match statements, we have the pattern:
// ( $( $x:expr),*)  and followed by => and the block of code that is generated for that pattern
// In this case, we only have 1 pattern, anything deviating from this pattern generates an error.
// Lets break down the syntax of the pattern:
//
// - We use parantheses to encompass a pattern
// - We use $ dollar sign to declare a variable: In this case we capture a variable $x which
// matches any Rust expression
// - The comma indicates that there must be commans between each instance of the pattern than is
// matched in the $(). In our case, Rust expressions.
// - The * specifies that the pattern matches zero or more of whatever preceded it
// Then in the body, the thing in between the $() expands and the $x is replaced for every
// expression matched in the pattern.
// So for example:
fn main() {
    let v1 = vec![1, 2, 3]; // this is the same as 
    let v2 = {
        let mut temp_vec = Vec::new();
        temp_vec.push(1);
        temp_vec.push(2);
        temp_vec.push(3);
        temp_vec
    };
    // this, again, isnt actually fully true, we just are omitting performance lines

    // There's a second type of macros, procedular macros
    // They act like a function, they take some code as input, operate on the code
    // and spit out some code as output.
    // When creating these macros, they need to be defined in their own crate with a
    // special crate type
    // There are 3 kinds of procedular macros:
    // - custom derive
    // - attribute-like
    // - function-like
    // Lets start with creating a custom derive, down below
}

pub trait HelloMacro {
    fn hello_macro();
}
// This is the trait we want to create a derive for
// We need to create a new macro in their own create
// This is a restriction in rust that eventually they hope to fix, but for now
// it's how we need to do things.
//
// Attribute like macros
// They look much like the custom derive, but rather than generate code for the attribute, we
// create new attribute. for example:
//#[route(GET, "/")]
//fn index() {}

// we cant use this since this isnt a lib and we cant use proc_macro
//#[proc_macro_attribute]
//pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
//
//Function like macros,
//Again very much like the other 2, we create macros that look like function calls
//let sql = sql!(SELECT * FROM posts WHERE id=1);
//
//#[proc_macro]
//pub fn sql(input: TokenStream) -> TokenStream {
