## Notes

A workspace is a set of package that share the same cargo.lock and output directory. 
If the projects get too big we can split them into separate crates essentially

Instead of a package section in Cargo.toml, we replace it with `[workspace]`. 
Also we add `resolver = "3"`, it's the 3rd version of an algorithm designed to help
put together the packages. 

Below resolver we have a members list where we add the packages we create into it.
