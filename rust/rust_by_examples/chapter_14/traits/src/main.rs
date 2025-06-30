// Traits are awesome I'll be honest
// Seems to be a really good alternative to inheritance
// And it works with generics, which is what we are covering today.

struct Empty;

// A trait generic
trait DoubleDrop<T> {
    // Define a method
    fn double_drop(self, _: T); // the `_` means variable without name, essentially we dont care
    // about one of the argument since the purpose of this function is to take ownership of both
    // self and the argument and drop them both, freeing their memory
}

// Implement doubledrop for any caller
impl<T, U> DoubleDrop<T> for U {
    fn double_drop(self, _: T) {}
}
fn main() {
    let empty = Empty;
    let empty2 = Empty;
    empty.double_drop(empty2);

    // empty; Dropped these so cant use them
    // empty2;
}
