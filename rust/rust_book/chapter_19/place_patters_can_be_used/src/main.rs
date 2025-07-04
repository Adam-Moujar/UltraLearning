// A bit odd to have this material now
// Patterns are special syntax for matching against structure of types
// Here's the ways we can use patters
fn main() {
    let x = Some(32);

    // match arms
    match x {
        // PATTERN => EXPRESSION,
        None => None,
        Some(i) => Some(i + 1),
    };

    // conditional if let
    let favourite_color: Option<&str> = None;

    //if PATTERN = VAL {
    //}else{}
    if let Some(color) = favourite_color {
        println!("Your favourite color is: {color}");
    } else {
        println!("You don't have a favorite color!");
    }

    // while let

    let mut count = Some(0);
    while let Some(num) = count {
        if num == 10 {
            count = None;
        } else {
            count = Some(num + 1);
        }
    }

    // for loops

    let v = vec!['a', 'b', 'c'];

    // this is a pattern (index, value)
    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }

    // The sneakiest pattern that we've been using
    // let

    let (x, y, z) = (1, 2, 3);
    // let pattern = expression;

    // and finally, function parameters
    let point = (3, 5);
    print_coordinates(&point);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current loc: ({x}, {y})");
}
