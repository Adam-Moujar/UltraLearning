// Just like ch2, this is a chapter where we use what we have learnt so far to tackle a project
// We will actually show a sneak peak of future chapter as well
// In this case, we are building a mini-grep like cli tool we can use to search for strings.
//
// Theres actually already a grep written in rust called ripgrep, which I use!

use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect(); // read command line arguments
    // args[0] is always the name of the program

    // closure covered in the next chapter
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
