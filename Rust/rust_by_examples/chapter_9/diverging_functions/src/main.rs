// Diverging functions never return, either due to a infinite loop
// or most likely if they are supposed to panic
// Diverging functions are annotated with ! to signify that they do not return
fn foo() -> ! {
    panic!("This call never returns.");
}
fn main() {
    println!("Hello, world!");
}
