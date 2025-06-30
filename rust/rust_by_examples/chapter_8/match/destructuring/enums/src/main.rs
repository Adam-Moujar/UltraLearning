enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
}
fn main() {
    let color = Color::RGB(122, 17, 40);

    match color {
        Color::Red => println!("The color is RED!"),
        Color::Blue => println!("The color is BLUE!"),
        Color::Green => println!("The color is GREEN!"),
        Color::RGB(r, g, b) => println!("Red: {r}, Green: {g}, Blue: {b}"),
        Color::HSV(h, s, v) => println!("Hue: {h}, saturation: {s}, value: {v}"),
    }
}
