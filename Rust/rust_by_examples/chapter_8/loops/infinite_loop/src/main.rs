// Rust has a loop keyboard which is just the same as while(1) in C
// The most common way to stop the look is with the break keyword

fn main() {
    let mut count = 0u32;
    loop {
        count += 1;

        println!("Current count val: {count}");

        if count == 1000000 {
            println!("Looped too much");
            break;
        }
    }
}
