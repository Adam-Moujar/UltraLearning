// sometimes we would love to give names to values in match arms so we can access them later,
// binding lets us do that
fn age() -> u32 {
    15
}
fn main() {
    match age() {
        0 => println!("Im just a baby"),
        n @ 1..=12 => println!("I'm a child of age {n:?}"),
        n @ 13..=19 => println!("I'm a child of age {n:?}"),
        n => println!("I'm an old person of age {n:?}"),
    }
}
