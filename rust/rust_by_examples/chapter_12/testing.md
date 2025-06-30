## Notes

2 types of test:

- Unit testing, testing each piece of functionality independently, will usually be in the same 
file as the functionality we are testing is defined
- Integration testing, testing normal app run, defined in a seperate file called tests 

A project directory will often look like:

foo
├── Cargo.toml
├── src
│   └── main.rs
│   └── lib.rs
└── tests
    ├── my_test.rs
    └── my_other_test.rs

To test, we simply run `cargo test` 
We can run specific tests with `cargo test pattern` to run tests that match that pattern. 
Cargo will run tests concurrently, make sure there's no race condition or anything that might mess up concurrency. 

