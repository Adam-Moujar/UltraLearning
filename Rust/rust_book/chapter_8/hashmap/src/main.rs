// Ending our exploration of rust collections we got hashmaps coming up.
// Stuff is pretty standard so as always I will only cover rust specific stuff
use std::collections::HashMap;

fn main() {
    // when you add a value into the hashmap, if its primitive it will copy it, otherwise it will
    // own it.
    let mut scores = HashMap::new();
    // owns the blue, copied the 10
    scores.insert(String::from("Blue"), 10);
}
