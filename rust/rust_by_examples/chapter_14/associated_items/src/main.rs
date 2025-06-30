// Theres a problem with generic types in traits
// The fact we need to specify the type to turn them from generic to concrete.
// This might seem like, well the point of it, but there are times where it is not needed.

// This is an example where it is not needed
struct Container(i32, i32);

// Contains is something that required A and B
//trait Contains<A, B> {
//    fn contains(&self, _: &A, _: &B) -> bool; // Explicitly requires `A` and `B`.
//    fn first(&self) -> i32; // Doesn't explicitly require `A` or `B`.
//    fn last(&self) -> i32; // Doesn't explicitly require `A` or `B`.
//}
//
//// But container has a i32, i32 defined in itself, so we dont need to
//// make i32 a concrete type, container itself contains i32, i32.
//impl Contains<i32, i32> for Container {
//    // True if the numbers stored are equal.
//    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
//        (&self.0 == number_1) && (&self.1 == number_2)
//    }
//
//    // Grab the first number.
//    fn first(&self) -> i32 {
//        self.0
//    }
//
//    // Grab the last number.
//    fn last(&self) -> i32 {
//        self.1
//    }
//}

// We can use associated types in these cases

trait Contains {
    // Associated types defined here
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {
    // Specify what types `A` and `B` are. If the `input` type
    // is `Container(i32, i32)`, the `output` types are determined
    // as `i32` and `i32`.
    type A = i32;
    type B = i32;

    // `&Self::A` and `&Self::B` are also valid here.
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }
    // Grab the first number.
    fn first(&self) -> i32 {
        self.0
    }

    // Grab the last number.
    fn last(&self) -> i32 {
        self.1
    }
}

fn main() {
    println!("Hello, world!");
}
