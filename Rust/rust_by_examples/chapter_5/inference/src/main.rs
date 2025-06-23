// Rust type inference works not just on current lines but future aswell
fn main() {
    let elem = 5u8; // Rust knows elem is a u8

    let mut vec = Vec::new(); // rust doesnt know what type of vec this is yet

    vec.push(elem); // But now it does
}
