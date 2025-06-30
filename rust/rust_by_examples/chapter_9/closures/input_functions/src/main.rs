// If you can have closures as input parameters can you have normal functions aswell?
// Yes!!!

fn call_me<F: Fn()>(f: F) {
    f();
}

// Define a wrapper function satisfying the `Fn` bound
fn function() {
    println!("I'm a function!");
}

fn main() {
    let closure = || println!("I'm a closure!");

    // Call me can use both a closure and a function as a input parameter
    call_me(closure);
    call_me(function);
}
