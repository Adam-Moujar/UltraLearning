// A gathering of all the syntax valid in patters
fn main() {
    // matching literals

    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Matching named variables
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case"),
    }

    // Multiple patterns
    let x = 1;

    match x {
        1 | 2 => println!("Could be one or two"),
        _ => println!("Not one or two"),
    }

    // A range of values
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("Not between 1 and 5 inclusive"),
    }

    // Destructing structs
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    //or

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    // or using match
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    // Destructing enums
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
    }

    // Destructing nested enums and structs
    let msg = NewMessage::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        NewMessage::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        NewMessage::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}");
        }
        _ => (),
    }

    // Some more structs and tuples
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    // Ignoring values using _
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    // or
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}");
        }
    }

    // creating unused variable with starting _

    let _x = 5; // No warning even though its unused;

    // Returning parts of value using ..
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {x}"),
    }

    let numbers = (2, 4, 8, 16, 32);

    //match numbers {
    //    (.., second, ..) => {
    //        println!("Some numbers: {second}")
    //    },
    //}
    // doesnt work since you cant have 2 ..
    // the compiler is not sure about how many to ignore

    // Extra conditions with match guards
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // @ bindings
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    // @ lets us save the variable we match to, in this case id_variable
    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {id_variable}"),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {id}"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
enum NewMessage {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}
