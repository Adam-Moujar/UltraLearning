// Result is kind of similar to option, but instead of using it when nothing is possible as a
// return, we use it when we expect errors.
use std::num::ParseIntError;
fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
    // Parse returns a result, if it can parse, it will return Ok(value)
    // otherwise it will Error since it cant parse it.
    // We use unwrap to get the value out of it, or panic if its an error
    // In production, it shouldnt be unwrap but for demostrative purposes this is good enough.
    let first_number = first_number_str.parse::<i32>().unwrap();
    let second_number = second_number_str.parse::<i32>().unwrap();
    first_number * second_number
}

// now we really should handle these errors, the main we do it is just returning it and hoping at
// some point up above the chain of functions we deal with the function.
// In reality, the way we should handle these is as quick as possible.
fn new_multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    match first_number_str.parse::<i32>() {
        Ok(first_number) => match second_number_str.parse::<i32>() {
            Ok(second_number) => Ok(first_number * second_number),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

// A lot of the functions for option can be used for result aswell
fn third_multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str
            .parse::<i32>()
            .map(|second_number| first_number * second_number)
    })
}

// Does it get really frustrating to keep writing Result<i32, ParseIntError> well I have the
// solution for you
// Use an alias for the type
type AliasedResult<T> = Result<T, ParseIntError>;

fn fourth_multiply(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
    Ok(0)
}

// Sometimes we want to stop a function and return an error, or continue if its an Ok
// For example if opening a file returns an error we stop the function
// otherwise we continue operating on the file.
// We can do this using early explicit returns
fn fifth_multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = match first_number_str.parse::<i32>() {
        Ok(first_number) => first_number,
        Err(e) => return Err(e),
    };

    let second_number = match second_number_str.parse::<i32>() {
        Ok(second_number) => second_number,
        Err(e) => return Err(e),
    };

    Ok(first_number * second_number)
}

// And finally the holy pinnacle of convinience, which to be honest might be too convinient, the ?
// operator. It just returns the error from the function or gets the value if its ok
// Look how simply that was man
fn last_multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = first_number_str.parse::<i32>()?;
    let second_number = second_number_str.parse::<i32>()?;

    Ok(first_number * second_number)
}

// Main can actually also return a Result
fn main() -> Result<(), ParseIntError> {
    Ok(())
}
