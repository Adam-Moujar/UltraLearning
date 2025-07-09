// Unit testing aims to test an isolated chunk of code, usually a function, at a time
// Integration testing is the opposite, testing that the library whole works
// Integrations testing resides in a file called tests and acts as if its an external crate.
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
