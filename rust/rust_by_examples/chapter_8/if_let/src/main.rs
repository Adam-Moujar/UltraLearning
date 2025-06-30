// If let is syntatic sugar for certain awkward enum use cases

//exercise
enum Foo {
    Bar,
}

fn main() {
    let optional = Some(7);
    match optional {
        Some(i) => println!("This is really long string and `{i:?}`"),
        _ => {} // you are required to have the _ path since match needs to cover every branch
                // but if you only want to use the some path you are stuck with a pretty awkward looking
                // match arm
    }

    // This is the same thing as the one above, just much more simpler and less awkward
    if let Some(i) = optional {
        println!("This is a really long string and `{i:?}`");
    }
    // We can even have a branch if the some(i) isnt true
    if let Some(i) = optional {
        println!("This is a really long string and `{i:?}`");
    } else {
        println!("Short string");
    }

    //exercise
    let a = Foo::Bar;
    if let Foo::Bar = a {
        println!("a is foobar");
    }
}
