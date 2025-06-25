// Lets talk about recoverable errors
//
// We handle them using Result, which is defined as:

// Its an enum where if the code successfuly works, sends back a Ok variant with some value if its
// expecting a value
// Otherwise, if it all goes wrong, we send back an err with an error message, and then decide how
// to handle the error.

use std::fs::File;
use std::io::{self, ErrorKind, Read};

//propagating error using ?
fn get_hello_msg() -> Result<String, io::Error> {
    let mut file = File::open("hello.txt")?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}

fn main() {
    // Common usage of result
    let greeting_file = File::open("hello.txt");

    let greeting_file = match greeting_file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => panic!("Problem opening the file: {error:?}"),
        },
    };
}
