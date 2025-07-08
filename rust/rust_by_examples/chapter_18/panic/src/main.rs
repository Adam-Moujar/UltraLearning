// Error handling is a fancy way of saying handling the points of failure in a program.
// The simplest for of error handling is to just panic! and let the program crash.
fn drink(beverage: &str) {
    // You shouldn't drink too many sugary beverages.
    if beverage == "lemonade" {
        // You can pass a message that will be displayed in the output
        panic!("AAAaaaaa!!!!");
    }

    println!("Some refreshing {} is all I need.", beverage);
}

fn main() {
    drink("water");
    drink("lemonade");
    drink("still water");
}
