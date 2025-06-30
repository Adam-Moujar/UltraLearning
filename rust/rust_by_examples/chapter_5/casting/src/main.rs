// Rust cant cast implicitly, but explicit type casting is allowed using as
fn main() {
    let decimal = 65.4321_f32;

    let integer = decimal as u8;
    let char = integer as char;
}
