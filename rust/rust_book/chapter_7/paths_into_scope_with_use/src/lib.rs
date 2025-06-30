// Writing the lengthy paths can get very repetitive and tiring,
// with the use keyword, we can shortcut alot of the work

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// after this, we dont need to write crate::front_of_house::hosting, only hosting.
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

// note that only things in the same as scope as the use can see it
//mod customer {
//    pub fn eat_at_restaurant() {
//        hosting::add_to_waitlist();
//    }
//}
// This wouldnt work since eat at restaurant is now under customer and so in a different scope
// So it actually doesnt know what hosting is;
//
// We could fix it by adding super::hosting on the mod
//
// Instead of bringing crate::front_of_house::hosting, couldnt we also add add_to_waitlist, so we
// dont need to write hosting::add_to_waitlist but rather just add_to_waitlist directly?
//
// Yes, but its idiomatic to actually write the parent, since:
//
// 1. its obvious it comes from a different file
// 2. in case you bring 2 different things with the same name, which is actually quite common
// in the case of bringing a custom result type, e.g fmt::Result and io::Result
//
// We could also give it a different name using the as keyword;

use std::fmt::Result;
use std::io::Result as IoResult;

// Re-exporting names with pub use
//
// using the keyword use makes the name private to the scope its imported in.
// If we want code outside the scope it was used to be able to refer to it,
// use pub use.
//
// Instead of writing use std::cmp and use std::io we could write
use std::{cmp::Ordering, io};
// this is called nested paths, and its just to save space and make it slightly more readable
//
//
// If instead you wanted to get everything from a path
use std::collections::*; // This gets everything in the scope of std::collections
// The glob operation (*) is mostly used in testing
