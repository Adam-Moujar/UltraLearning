// Haven't covered dyn yet in rust book so I am going in semi blind
//
// When returning a type, the function needs to know how much space the type occupies in space
// Now here's an exercise, if we have a trait Animal , and a function that returns a type that
// implements Animal how much space does the function need?
//
// ...
//
// The answer is, who knows, many different types can implement Animal, each with their
// different sizes, and so the compiler has no idea which of these you want to return
//
// This is where dyn comes in, dynamic, we wrap it in a Box<dyn Animal>, essentially a pointer
// to a dynamic animal and thats what we return.
//
// The pointer has a fixed space so the function is satisfied, and the heap is built for runtime
// memory initialisation, so the world is happy.
struct Sheep {}
struct Cow {}

trait Animal {
    // Instance method signature
    fn noise(&self) -> &'static str;
}

// Implement the `Animal` trait for `Sheep`.
impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}

// Implement the `Animal` trait for `Cow`.
impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}

// Returns some struct that implements Animal, but we don't know which one at compile time.
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

fn main() {
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!(
        "You've randomly chosen an animal, and it says {}",
        animal.noise()
    );
}
