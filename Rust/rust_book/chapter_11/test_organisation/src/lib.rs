// Oh boy, testing is a complex discipline, one that admittedly, I need to
// get better at.
//
// Testing is split into 2 categories:
// 1. Unit testing
// 2. Integration tests
//
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// Unit testing
// Testing each unit of code in isolation
// The things we were discussing before were examples of unit testing
// cfg tells rust that when someone runs the command
// cargo test
// the code will run and not when you run cargo build
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
// Integration tests
// we create a seperate folder in the root directory called tests, and we put our integration tests
// in that folder.
// We need to add use to bring the functionality into scope
// and also annote it with #[test]
// after that, we can test normally and run using cargo test
// Surprisingly, we dont need the cfg(test) because rust automatically knows
// that files in the test folder means they are tests
// We can run only integration tests with cargo test --test integration_test
