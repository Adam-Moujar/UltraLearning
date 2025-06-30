// By default, variables are immutable, meaning once defined, they cannot change values.
// To make a variable mutable however, is quite easy. You just need to add mut keyword before.
fn main() {
    let x = 5; // automatically immutable
    println!("The value of x is: {x}");
    // x = 6; // This does not compile since x is immutable and cannot change.
    println!("The value of x is: {x}");

    let mut y = 5; // manually specified its mutable
    println!("The value of y is: {y}");
    y = 6; // This compiles now since y is mutable and can change.
    println!("The value of y is: {y}");

    // We also have const keyword. The difference is that constants are always immutable period.
    const HOURS_IN_DAY: u32 = 60 * 60 * 24;

    // We can also shadow immutable variables, create a new variable with the same name as another
    let a = 5;

    let a = a + 1;

    {
        let a = a * 2;
        println!("The value of a in the inner scope is: {a}");
    }
    println!("The value of a in the outter scope is: {a}");
    // This functionality is mostly used when handling type changes like
    // in the example in ch2 where we had a string and we parsed it into int
    //
    // Or if we have a variable that undergoes once change straightaway and then is immutable.
}
