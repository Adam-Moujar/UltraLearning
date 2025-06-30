## Notes

In quite complex situations, normal cargo build is not enough. 
To circumvent this, you can add `[package]` section to cargo.toml where 
we specify a build package called build.rs, and that is where we put our build scripts. 

