// Modules by default, make their items private. If you want to make them public use the
// pub keyword

mod my_mod {
    fn private_function() {
        println!("called `my_mod::private_function()`");
    }

    pub fn public_function() {
        println!("called `my_mod::public_function()`");
    }

    // Items in the same module can access things even if they are private
    pub fn indirect_access() {
        print!("called `my_mod::indirect_acces()`, that\n>");
        private_function();
    }

    // Nested modules, idk how useful this is tbh
    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my_mod::nested::private_function()`");
        }

        // Functions declared using `pub(in path)` syntax are only visible
        // within the given path. `path` must be a parent or ancestor module
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            print!("called `my_mod::nested::public_function_in_my_mod()`, that\n> ");
            public_function_in_nested();
        }
        // Functions declared using `pub(self)` syntax are only visible within
        // the current module, which is the same as leaving them private
        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested()`");
        }
    }
    pub fn call_public_function_in_my_mod() {
        print!("called `my_mod::call_public_function_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
    }
}
fn main() {
    // my_mod::private_function(); // Cant because its private
    my_mod::public_function(); // Can because its public
    my_mod::indirect_access();
    // my_mod::public_function_in_my_mod() //Only accessible to things in my mod
    my_mod::call_public_function_in_my_mod();
}
