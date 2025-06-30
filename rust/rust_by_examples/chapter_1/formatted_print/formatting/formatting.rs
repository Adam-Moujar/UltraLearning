// Formatting is handled via a format string e.g:
// "{:X}" for hex etc...
//

// Exercise given by the book
use std::fmt::{self, Display, Formatter};
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "RGB ({}, {}, {}) 0x{:0>6X}",
            self.red,
            self.green,
            self.blue,
            self.red as u64 * 65536 + self.green as u64 * 256 + self.blue as u64
        )
    }
}

fn main() {
    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ] {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        println!("{}", color);
    }
}
