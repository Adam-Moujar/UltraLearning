fn main() {
    let reference = &4;

    match reference {
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // you can also dereference it
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    // you can do the opposite, go from value to reference
    let value = 5;

    match value {
        ref r => println!("Got a reference to a value: {r:?}"),
    }
}
