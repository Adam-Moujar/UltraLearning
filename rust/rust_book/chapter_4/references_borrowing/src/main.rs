// the ownership leads to a quite awkward problem in Rust,
// if you want to pass a value to a function but still use it after
// the function you'll need to return the value, otherwise the function will
// drop it since it owns it.

// Borrowing solves this issue by giving the function a reference, a way to
// use the value, without owning it.

fn test_borrowing(s: &String) -> String {
    format!("{}{}", s, ", world!")
}

fn test_mutable_borrow(s: &mut String) {
    s.push_str(", world!");
}
fn main() {
    // This is borrowing, where we pass a reference rather than the value.
    let s = String::from("hello");
    println!("s: {s}, func: {}", test_borrowing(&s));

    // Just like variables, borrows can be mutable, so we can
    // change the underlying value of the variable we are borrowing.
    // Both the variable and the reference has to be mutable. Otherwise, no can do.
    let mut s1 = String::from("hello");
    test_mutable_borrow(&mut s1);
    println!("new s: {s1}");

    // Mutable references come with a biiiiiiig restriction,
    // you can only have one reference active, if there's a mutable reference
    // With non-mutable, you can have many different refences active in comparison

    let s2 = String::from("Test");
    let ref1 = &s2;
    let ref2 = &s2;
    // This is ok since we can have multiple ref if none of them are mutable

    let mut s3 = String::from("Test");
    let r1 = &mut s3;
    //let r2 = &mut s3;
    //let r2 = &s3;
    //println!("{}, {}", r1, r2);
    // Can't do this, wont work.
}
