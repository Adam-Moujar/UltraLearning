// Get the io bundle in std so we can handle input
use std::io;

// This use gets us the comparison orderings
use std::cmp::Ordering;

// Get the external rand createa nd get the rng bundle to get random numbers
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // This generates a random number given a range, in this between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        // Initialise a new mutable string that's empty for now
        let mut guess = String::new();

        // Read a line into guess, and if it goes wrong, print "Failed to read line"
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // When reading input, its read as a string, now we convert to a number while handling possible
        // errors
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // We compare guess and secret number and we match based on the result
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too big!"),
        }
    }
}
