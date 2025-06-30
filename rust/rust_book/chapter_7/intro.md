## Notes

As programs grow langer, splitting the code into seperate files is advisable
In rust you can have many different binary crates, but one library crate.

Rust's module system includes:
- Packages: A cargo feature that lets you build, test and share crates
- Crates: A tree of modules that produces a library or executable
- Modules and use: Let you control the organization, scope and privacy of paths
- Paths: A way of naming an item, such as strut, function or module

Encapsulation is something that needs to be in the developers mind, to make sure
that each crate and module has a public interface to work with, without being bogged down with the
implementation details. 

Scope also refers to the context surrounding a piece of code, all the code, variables, functions
that a certain section of the code knows and has available to use.


