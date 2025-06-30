// This is the most basic control flow and it's in every language I have used.
//
// only quirk is no parenthesis around the boolean expression
fn main() {
    let n = 5;

    if n < 0 {
        println!("{n} is negative");
    } else if n > 0 {
        println!("{n} is positive");
    } else {
        println!("{n} is zero");
    }
}
