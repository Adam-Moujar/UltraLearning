// The purpose of generics is to generalise, structs, functions, etc..., that can take in many
// different types in just one function
//
// This is how you define a generic function, where T is any type
// Currently this wont compile, because not any function can be ordered using >
// only certain types like characters and integers can.
//
// We will fix this function later when we learn about traits.
//
//fn largest_i32<T>(list: &[T]) -> &T {
//    let mut largest = &list[0];
//
//    for item in list {
//        if item > largest {
//            largest = item;
//        }
//    }
//
//    largest
//}
fn main() {
    println!("Hello, world!");
}
