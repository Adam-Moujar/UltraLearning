// crate_type is an attribute that tells the compiler if a crate is a binary or library
// not that useful if you use cargo since it does that for you, but well you know, it's
// useful to have the ability to tell rustup if it is a library or not ahead of time
#![crate_type = "lib"]
#![crate_name = "rary"]

pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}
