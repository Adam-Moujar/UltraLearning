pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// As a base, tests run in parallel
// So make sure that all the headaches with parallel code
// Race conditions, sharing data, etc... dont happen
// If you want to only run it with one thread use
// cargo test -- --test-threads=1
// If a test succeds, it doesn't show any output, like println!
// If you want to always show the output run the command
// cargo test -- --show-output
// (I'll be honest, that last command seems pretty darn useless)
// To run only certain tests, we can filter by name
// run the command
// cargo test three
// to only run tests with three in the name
// (This is probably the most useful controlling we can do)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_three_and_two() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore] // Makes sure the test doesn't run, useful if the test takes tooo long
    // If you want to run ignored tests only
    // cargo test -- --ignored
    // If you want to run all tests, ignored or not ignored
    // cargo test -- --include-ignored
    fn test_three_and_four() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
