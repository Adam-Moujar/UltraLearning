// The convention for naming functions is snake_case
//
// Where functions are defined does not matter, any function can call
// any other function so long as they can see it.

// Weirdly, a function returns the last statement it sees
// so long as it has no semicolon after it
// We can also explicitly return using the return keyword
fn area_of_circle(radius: f64) -> f64 {
    const PI: f64 = 3.141592654;

    PI * radius * radius // This returns it 
    // return PI * radius * radius; These 2 statements are the same
}
fn main() {
    println!("Hello, world!");
}
