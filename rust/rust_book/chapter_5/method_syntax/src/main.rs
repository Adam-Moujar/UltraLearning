// Methods in rust reside in implementation blocks rather than in struct definitions.
struct Rectangle {
    width: u32,
    height: u32,
}

// impl block for rectangle
impl Rectangle {
    // define a function which takes a reference to the current object and returns a u32
    // this is interesting since &self means we can use the . notation for the function
    // object.method is the same as method(object)
    // also since its a reference, this is a borrow, so we can still use the object after the call
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // technicaly not a method since no self.
    // Self refers to current object type
    // self refers to current object
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
