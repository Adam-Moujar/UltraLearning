// Strings will be the second collection to consider, and it is quite a bit more complicated than
// usual
//
// String is implemented as a wrapper around vector of bytes
fn main() {
    let mut s = String::new();

    let s = "hi".to_string();

    // Strings start to get weird when we try to retrieve the data.
    //let h = &s[0]; //doesnt work
    //The reason why is about the depth of how strings are stored. Since they are stored as bytes,
    //diferrent characters actually have different sizes in bytes, and so when you access the first
    //element for example there's some ambiguity, did you mean the first character? or the first
    //byte?
    //
    //So if you want chars
    for c in "Зд".chars() {
        println!("{c}");
    }
    // and if you want bytes
    for b in "Зд".bytes() {
        println!("{b}");
    }
}
