// You can create a library crate (like this one) by running the command cargo new `name` --lib.
//
//pub fn add(left: u64, right: u64) -> u64 {
//    left + right
//}
//
//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn it_works() {
//        let result = add(2, 2);
//        assert_eq!(result, 4);
//    }
//}
// This is the default code in a library crate.
//
// To tell rust where to find an item in a module tree we use a path, and just like in a
// filesystem, a module system path can take 2 forms:
//
// 1- absolute path, starting from the crate root, for external crate its a path beginning with the
// crate name, e.g std::etc..., and for the current cratem it starts with the literal crate::
//
// 2- relative path, starts from the current module and uses self, super or something lese in the
// current module
//
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    //absolute path
    //crate::front_of_house::hosting::add_to_waitlist();

    //relative path
    //front_of_house::hosting::add_to_waitlist();
    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("wheat");

    println!("I'd like {} toast please", meal.toast);
}

// absolute path or relative path depends, on whether or not the code will move together(relative
// path pro) or will likely separate into files (absolute path pro)
//
// To make something in a module public, you also need to make the module itself public
//
// siblings can also refer to each other without the pub use

fn deliver_order() {}

// using super for relative. Super looks at the scope of the parent module, in this case back of
// house which sees everything.
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    // For a struct, each field needs to be made public, for the filed to be public to other
    // modules
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

// in enums however, only the enum needs to be pub, the variants will be automatic public aswell.
