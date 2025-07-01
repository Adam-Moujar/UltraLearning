// Lifetimes seem like something that will become a problem.
// I've heard whispers of the pain of lifetimes and async, but what do I know?
//
// Anyway: lifetimes are there to make sure borrows are valid.
// If the lifetime of a borrow is longer than the lifetime of what it borrows, we got biig problems
// Its fine to coerce a borrow into a shorter lifetime, but going longer than it is allowed? Nope
//
// So we can give explicit annotations when it is needed to make sure it doesnt happen
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

fn failed_borrow<'a>() {
    let _x = 12;

    // let _y: &'a i32 = &x;
    // Doesnt work because the lifetime of _x is function bound, but _y 'a means its longer than x
    // which again, is a biiiig nono
}
// There are some rules to remember
// Any reference as an argument to a function must be annotated (except in some cases)
// Any reference being returned must have the same lifetime as an input (except some cases) or 'static
//
// The cases are when the compiler can infer what lifetime the references should be.

// Structs also need lifetimes, for example this isn't allowed
//struct NamedBorrowed {
//    x: &i32,
//}

// This is how it should be
struct NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}

// And yes you guessed it, so do traits, they need lifetimes too.
// This is a trait for default values
impl<'a> Default for NamedBorrowed<'a> {
    fn default() -> Self {
        Self { y: &1, x: &2 }
    }
}

// We can get sillier, if traits/generic types can be bounded, can lifetimes be too?
// Yes
struct Ref<'a, T: 'a>(&'a T);
// `Ref` contains a reference to a generic type `T` that has
// some lifetime `'a` unknown by `Ref`. `T` is bounded such that any
// *references* in `T` must outlive `'a`. Additionally, the lifetime
// of `Ref` may not exceed `'a`.

// A generic function which prints using the `Debug` trait.
use std::fmt::Debug;
fn print<T>(t: T)
where
    T: Debug,
{
    println!("`print`: t is {:?}", t);
}

// Here it is, a generic Type T bounded to beind Debug and have a lifetime 'a
fn print_ref<'a, T>(t: &'a T)
where
    T: Debug + 'a,
{
    println!("`print_ref`: t is {:?}", t);
}

fn main() {
    let (four, nine) = (4, 9);

    print_refs(&four, &nine);
}
