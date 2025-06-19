// While debug comes with the benefit of being automatically
// generated using the derive Debug functionality.
// The lack of customisation makes it lacking
//
// Thus we can use, fmt::Display, which uses the {} print marker.

// We need this to make the fmt::Display trait available;
use std::fmt;

struct Structure(i32);

impl fmt::Display for Structure {
    // To implement the trait, we must implement this exact function, with this exact signature
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // f is the output stream we need to write our string into.
        // Returns fmt::Result which indicates if it worked or not,
        // This writes into f, self.0, which in our case is the number.
        write!(f, "{}", self.0)
    }
}

// Other examples of implementing display

struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

// Exercise given by rust by examples

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

// Implementing vectors, or collections in general is a bit tricky.
// If any of the elements returns an error, we must handle them.
// Rust has an operator ? for this exact reason, if it's an error it will return it
// Otherwise it just continue

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{count}: {v}")?;
        }

        write!(f, "]")
    }
}
fn main() {
    println!("{}", Structure(1));
    println!("{}", Point2D { x: 1.0, y: 2.0 });
    // Answer to exercise
    println!(
        "Display: {}",
        Complex {
            real: 3.3,
            imag: 7.2
        }
    );

    println!(
        "Debug: {:?}",
        Complex {
            real: 3.3,
            imag: 7.2
        }
    );

    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
