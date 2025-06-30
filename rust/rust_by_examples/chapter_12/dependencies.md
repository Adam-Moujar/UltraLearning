## Notes

Cargo manages dependencies for us, which coming from c, thank god for that. 
To add a dependecy it is as easy as adding a `[dependencies]` section in Cargo.toml
and adding the packages with their version under that section. e.g

`[dependencies]`
clap = "2.27.1" # from crates.io
rand = { git = "https://github.com/rust-lang-nursery/rand" } # from online repo
bar = { path = "../bar" } # from a path in the local filesystem

We can also do `cargo add package` which will add it to us with the latest stable version.

