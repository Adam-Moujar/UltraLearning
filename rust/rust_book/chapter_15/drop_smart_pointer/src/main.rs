// The next trait that is quite cool in smart pointers is the drop trait.
//
// Just like for deref, drop comes with smart pointer.
// But if you wanted to make your own

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        Self(x)
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping mybox");
    }
}
fn main() {
    let test = MyBox::new(5);
    // test.drop(); we cant explicitly run drop. This is because rust runs a drop for us, and if we
    // drop early, then rust would free after drop, which isa big nono.
    // We can drop using std::mem::Drop tho
    // It also runs for us if we comment the line below when we run cargo run. Since rust again,
    // runs drop at the end of the program scope.
    std::mem::drop(test);
}
