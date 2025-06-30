## Notes

After you complete your project, you can upload a crate to crate.io so other people can use it. 
There are some steps you have to take before uploading however.

### Making useful documentation comments

Rust has a special type of comment, documentation comments. They will be used to generate 
documentation using cargo doc --open. 

Documentation comments begin with /// e.g:

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

You must place documentation comments before the item you are documenting. 

#### Commonly used sections

In the above example we have a examples section, there's more sections that will be useful if shared:

- Panics: The scenarios in which the function being documented could panic. Callers of the function who don’t 
 want their programs to panic should make sure they don’t call the function in these situations
- Errors: If the function returns a Result, describing the kinds of errors that might occur and what conditions might cause 
those errors to be returned can be helpful to callers so they can write code to handle the different kinds of errors in different ways.
- Safety: If the function is unsafe to call (we discuss unsafety in Chapter 20), there should be a section explaining why the 
function is unsafe and covering the invariants that the function expects callers to uphold.

#### Documentation comments as tests

Example blocks in documentation comments can actually be used as special tests scalled doc tests. 
Running `doc-tests crate` will run the examples to make sure they are right

#### Commenting contained items

If we are trying to add doc comments but not for a specific item we use `//!`
It is mostly useful in the crate root file to explain the whole of a crate, or a module. 

### Exporting a Convenient Public API with pub use 

We covered how to do documentation, the next part which is important in open source crates is to make sure
that you provide a public api that users can use with the usage of `pub` and `use` and `pub use`.

One problem that happens, is that the structure you think is useful when developing the project, is often not 
something that is intuitive to other users. If you have a data type nested in a lot of modules that is quite useful,
other users might not even know it exists. 

We can use `pub use` to circumvent this problem. When you pub use these useful nested types. They will be noted in 
the documentation, as a way to give information to users about very useful types and functions etc... upfront.

### Setting up a crates.io account

I think I set one up already when I first went through the rust book, but well to upload to crates.io you need a crates.io account.
You can simply log in with github. 
After that you can use cargo login with a api key provided in crates.io/me to log in to crates.io on cargo. 

### Adding metadata to a new crate. 

When attempting to publish using `cargo publish`, you will find that you need a lot of information in your cargo.toml before you can do it. 
Make sure to write the:
- Description
- License or license-file if its not a license used in SPDX.
- Edition, I think this one is optional

### Publising a crate

You can just publish using `cargo publish` and voila!

### Publishing a new version

Make sure you change the version in cargo.toml, adhering to the semantic versioning rules, and just publish again.

### Deprecating versions from crates.io 

If a previous crate is broken for some reason, you can use cargo yank with the version number to make sure that 
no other project in the future can depend on it. 
You can't remove it, once an open source project is uploaded to crates.io it stays there, we can simply warn people
that it is deprecated and move on. e.g
`cargo yank --vers 1.0.1`
