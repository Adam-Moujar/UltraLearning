// Test functions go into the module tests with a cfg test
// running cargo test will run all the test functions
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
#[allow(dead_code)]
fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}

fn sqrt(number: f64) -> Result<f64, String> {
    if number >= 0.0 {
        Ok(number.powf(0.5))
    } else {
        Err("negative floats don't have square roots".to_owned())
    }
}

// sometimes a function does want to panic and the test should reflect that
pub fn divide_non_zero_result(a: u32, b: u32) -> u32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    } else if a < b {
        panic!("Divide result is zero");
    }
    a / b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_bad_add() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(bad_add(1, 2), 3);
    }
    // as of rust 2018, we can actually return Result from test
    // meaning we can also use ? for convinience
    #[test]
    fn test_sqrt() -> Result<(), String> {
        let x = 4.0;
        assert_eq!(sqrt(x)?.powf(2.0), x);
        Ok(())
    }
    // we can test whether a test that should panic, does
    #[test]
    #[should_panic]
    fn test_any_panic() {
        divide_non_zero_result(1, 0);
    }

    #[test]
    #[should_panic(expected = "Divide result is zero")]
    fn test_specific_panic() {
        divide_non_zero_result(1, 10);
    }

    // if we want specific tests we pass an extra argument during cargo test
    // and rust will match every test whose name contains that argument

    // if we want to ignore tests
    #[test]
    #[ignore]
    fn ignored_test() {
        assert_eq!(add(0, 0), 0);
    }

    // and if we specificly want to run ignored tests we run cargo test --ignored
}
