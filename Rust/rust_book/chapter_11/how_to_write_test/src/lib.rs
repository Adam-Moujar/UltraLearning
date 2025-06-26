pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// Here's a test, lying in plane sight
// To be fair, its right there
#[cfg(test)]
mod tests {
    use super::*; // This is here because of the visibility rules in ch7
    // Since we are inside a module, normally we wouldnt be able to see the functions, mods,
    // structs defined outside of this module.
    // super::* brings all of those into scope, so its visible to the tests

    // This annotation indicates that the function is a test function
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4); // A macro that panics if the assertion is false, in this case that
        // result isnt 4
    }

    #[test]
    fn fail_test() {
        panic!("Make this test fail"); // A panic makes the function automatically fail
    }
    #[test]
    fn assert_message() {
        let result = add(2, 3);
        assert_eq!(result, 4, "Result was not 4, the value was {result}"); // we can give assert a message in case it fails to
        // illucidate why it failed
    }
    #[test]
    // If the test should panic, this annotations will indicate that
    // and make sure the test passes if it panics
    #[should_panic]
    fn should_panic_check() {
        panic!("Make this test fail?");
    }

    // We can also returns results instead of panic, or asserting
    fn result_test() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}

// To run tests, we call cargo test instead of cargo run
