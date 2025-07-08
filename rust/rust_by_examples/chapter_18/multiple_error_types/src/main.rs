// Our previous examples were pretty convenient, we weren't mixing options and results
// and the error type for the results was the same, its not always like this, for example:
use std::num::ParseIntError;
fn double_first(vec: Vec<&str>) -> i32 {
    // These 2 give different error types
    let first = vec.first().unwrap(); // Generate error 1
    2 * first.parse::<i32>().unwrap() // Generate error 2
}

// The most basic fix for the fix issue, mixing options and results is to just embed them
fn second_double_first(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
    // vec first returns an option
    // parse returns an error
    vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n))
}

// If we want to swap the option and result we can use transpose
fn third_double_first(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n));

    opt.transpose()
}

// if we have different types of errors, we can mask them using a custom error type
#[derive(Debug, Clone)]

// our custom error type
struct DoubleError;

impl std::fmt::Display for DoubleError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "invalid first item to double")
    }
}

fn fourth_double_first(vec: Vec<&str>) -> Result<i32, DoubleError> {
    vec.first()
        // Change the error to our new type.
        .ok_or(DoubleError)
        .and_then(|s| {
            s.parse::<i32>()
                // Update to the new error type here also.
                .map_err(|_| DoubleError)
                .map(|i| 2 * i)
        })
}

// Another fix to the issue of multiple error types is to use Box Dyn Error
// this will mean we have to implement the error trait for our custom error

#[derive(Debug, Clone)]
struct EmptyVec;

impl std::fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "invalid first item to double")
    }
}

// Thankfully implement Error is incredibly easy
impl std::error::Error for EmptyVec {}

fn fifth_double_first(vec: Vec<&str>) -> Result<i32, Box<dyn std::error::Error>> {
    vec.first()
        .ok_or_else(|| EmptyVec.into()) // Converts to Box
        .and_then(|s| {
            s.parse::<i32>()
                .map_err(|e| e.into()) // Converts to Box
                .map(|i| 2 * i)
        })
}

// Turns out the ? operator is actually even better than I thought.
// It doesnt just return the error, it will actually also convert the return type to match the
// return type of the function, meaning we can use ? when we return Box dyn
fn sixth_double_first(vec: Vec<&str>) -> Result<i32, Box<dyn std::error::Error>> {
    let first = vec.first().ok_or(EmptyVec)?; // Look how convenient this thing is, oh my god
    let parsed = first.parse::<i32>()?;
    Ok(2 * parsed)
}

// And the final thing we can do when we have multiple error types, very similar to the struct
// solution, is to wrap them in an Enum
#[derive(Debug)]
enum DoubleErrorEnum {
    EmptyVec,
    Parse(ParseIntError),
}

impl std::fmt::Display for DoubleErrorEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            DoubleErrorEnum::EmptyVec => write!(f, "please use a vector with at least one element"),
            // The wrapped error contains additional information and is available
            // via the source() method.
            DoubleErrorEnum::Parse(..) => {
                write!(f, "the provided string could not be parsed as int")
            }
        }
    }
}

impl std::error::Error for DoubleErrorEnum {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            DoubleErrorEnum::EmptyVec => None,
            // The cause is the underlying implementation error type. Is implicitly
            // cast to the trait object `&error::Error`. This works because the
            // underlying type already implements the `Error` trait.
            DoubleErrorEnum::Parse(ref e) => Some(e),
        }
    }
}

// Implement the conversion from `ParseIntError` to `DoubleError`.
// This will be automatically called by `?` if a `ParseIntError`
// needs to be converted into a `DoubleError`.
impl From<ParseIntError> for DoubleErrorEnum {
    fn from(err: ParseIntError) -> DoubleErrorEnum {
        DoubleErrorEnum::Parse(err)
    }
}

fn last_double_first(vec: Vec<&str>) -> Result<i32, DoubleErrorEnum> {
    let first = vec.first().ok_or(DoubleErrorEnum::EmptyVec)?;
    // Here we implicitly use the `ParseIntError` implementation of `From` (which
    // we defined above) in order to create a `DoubleError`.
    let parsed = first.parse::<i32>()?;

    Ok(2 * parsed)
}

fn main() {
    println!("Hello, world!");
}
