// We've talked extensively about options TL:DR
// if there's a possibility the function returns nothing, we use Option
// If it can go wrong and error, we use Result

// It makes sense to use Option here, we dont want the program to error, we are simply saying we
// cant divide these 2
fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        // Failure is represented as the `None` variant
        None
    } else {
        // Result is wrapped in a `Some` variant
        Some(dividend / divisor)
    }
}

// we can use options with the match expression, or using their built-in functions
fn try_division(dividend: i32, divisor: i32) {
    // `Option` values can be pattern matched, just like other enums
    match checked_division(dividend, divisor) {
        None => println!("{} / {} failed!", dividend, divisor),
        Some(quotient) => {
            println!("{} / {} = {}", dividend, divisor, quotient)
        }
    }
}
fn main() {
    try_division(4, 2);
    try_division(1, 0);
}
