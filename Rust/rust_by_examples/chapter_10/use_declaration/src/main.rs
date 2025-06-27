// Use is pretty useful as a way to bind a path so its easier to access
mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`");
        }
    }
}

fn main() {
    deeply::nested::function();

    use deeply::nested;
    nested::function(); // This is the main usage of shortening names
    use deeply::nested as ns; // Can also give them different names
    ns::function();
}
