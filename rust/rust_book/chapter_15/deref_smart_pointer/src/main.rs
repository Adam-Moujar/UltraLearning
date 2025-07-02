// Deref trait is what allows smart pointers to act, just like normal pointers
//
// For a moment, lets just see how normal pointers work
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // And this works perfectly fine. How about smart pointers?
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Works just as well, and this is all thanks to the magic of the deref * operator
    //
    // If we wanted to make our own smart pointer
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }
    // This all makes sense
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    //assert_eq!(5, *y); // This doesnt work, we cant deref mybox yet, it hasnt implemented the
    //trait yet.

    use std::ops::Deref;
    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); //  it does now since we implemented deref (turns out impl works in the whole
    //  scope so the code above actually does work, but just pretend it doesnt)
    //
    // Certain deref implementations actually coerce into other types like the string type which
    // actually coerces into &str, string literal type.
}
