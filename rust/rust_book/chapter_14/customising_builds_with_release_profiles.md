## Notes

Cargo has 2 main release profiles, configurations that you can change, to change the way the compiler builds the project:

1. Dev, the build you get when you run cargo build, meant for debugging and development. 
2. Release, the build you get when you run cargo build --release, meant for actual usage and is much faster. 

You can add `profile.[name_of_profile]` sections to Cargo.toml to specify a custom profile.
The default values of opt-level, optimisation level, for dev and release profiles are:

[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3

3 being the highest opt-level possible. 
