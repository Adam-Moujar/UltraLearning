// It is very similar to from and into, except they return a result instead of the value
// The idea is if it can't convert then you return the err, otherwise you return the value

struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = (); // You need to implement the error type

    // try_from function signature
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}
fn main() {
    println!("Hello, world!");
}
