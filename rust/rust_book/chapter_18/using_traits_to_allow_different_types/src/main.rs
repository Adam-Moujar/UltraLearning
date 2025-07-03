// We've mentioned that vectors can onlu hold a element of a single type
// We could circumvent this using vector of enums which hold different data, e.g

enum Data {
    Integer(u32),
    String(String),
    Bool(bool),
}

// This is good if we know the different types of data we need to store, in this case
// u32, Strings and bools.
// But when we might an extended set of types, we can do something different
// We will make a mock draw library that can hypothetically have as many types as we want
// that as long as they implement a trait called Draw, they can be used to paint
pub trait Draw {
    fn draw(&self);
}

// The screen we draw the types that implement draw
pub struct Screen {
    // The different shapes that occupy the screen
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    // The function that draws all the components to the screen
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to draw a button
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

fn main() {
    let v = vec![
        Data::Integer(12),
        Data::String("Hi".to_string()),
        Data::Bool(true),
    ];

    // Now someone can and use different types, SelectBox, Button and draw them to the screen
    // And these types are unlimited so long as we implement them
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}

// Now this comes with tradeoffs, 1. someone has to implement the traits, but thats hardly an issue
// There's real performance cost when taking this approach.
// Versus the enum approach that we used earlier, however that cant be used in all cases.
// It's something to consider however
