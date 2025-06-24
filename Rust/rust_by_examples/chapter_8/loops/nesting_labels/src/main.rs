// This is something I'm personally not a big fan of, kind of like jmp and labels in C
// We can give loops labels and break loops using their label name
fn main() {
    // 'outer is the loop label name
    'outer: loop {
        println!("Entering the outer loop");

        'inner: loop {
            println!("Entering the inner loop");

            //break // this breaks current loop, so inner
            break 'outer; // breaks the outer loop
        }
        println!("We dont reach this");
    }
    println!("Exited outer loop");
}
