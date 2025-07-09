// Dev dependencies are dependencies only needed during development time, usually for testing or
// debugging code.
// One example is pretty_assertion which extends the assert_eq! used in tests to provide colors.
// These go in the Cargo.toml file
// you can also use cargo add `crate` --dev
fn main() {
    println!("Hello, world!");
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq; // crate for test-only use. Cannot be used in non-test code.

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
