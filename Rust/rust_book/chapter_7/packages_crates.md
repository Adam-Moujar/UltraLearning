## Notes

A crate is the smallest amount of code that the rust compiler considers at a time. 
Crates can contain modules which are defined in other files. 

A crate has 2 types: 
- Binary crates: programs that compile to an executable and you can run. 
- Library crates: they define functionality meant to be shared with projects, like the rand crate. 

The crate root refers to the file that the compiler starts from and it's also the root module of the crate. 

A package is a bundle of crates that provide a functionality. 
A package can have as many binary crates as you want, but only one library crate.
