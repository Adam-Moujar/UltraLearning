extern crate rary;
//using the external library rary
// We tell rustc later what rary is using command:
// rustc test.rs --extern rary=library.rlib &&
fn main() {
    rary::public_function();

    // Error! `private_function` is private
    //rary::private_function();

    rary::indirect_access();
}
