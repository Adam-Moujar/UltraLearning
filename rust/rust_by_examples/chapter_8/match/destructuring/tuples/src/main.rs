// Destructing is when a type with multiple values is broken down into each of its individual parts
// tuples being one of them
fn main() {
    let triple = (4, -2, 4);

    println!("Tell me about {triple:?}");

    match triple {
        // the tuple needs to have 3 values and a zero at the start, and you extract the value of the other 2
        (0, y, z) => println!("The first value is `0`, `y` is: {y:?}, and `z` is: {z:?}"),
        // the tuple has a 1 as the starting value, and you don't care for the rest
        (1, ..) => println!("The first value is 1 and I don't really care for the rest"),
        // the tuple has a 2 at the end and you don't care for the rest
        (.., 2) => println!("The last value is 2 and I don't really care for the rest"),
        (3, .., 4) => println!("The first is 3, the last is 4, the rest dont matter"),
        _ => println!("Binds to everything"),
    }
}
