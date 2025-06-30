// Attributes is metadata for a module, crate or item.
// They have a loooot of usages.
// Attributes look like `#[attribute_name]` or #![inner_attribute], the differences being:
//
// 1. #[outer_attributes] apply to the item immediately following it.
// 2. #![inner_attribute] applies to the entire scope
//
// Attributes which need arguments can be passed values
// #[attribute = "value"]
// #[attribute(key = "value")]
// #[attribute(value)]

// outer_attribute which we pass a value into
#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle {
    width: u32,
    height: u32,
}

// One of the most useful attributes that  you will use is allow dead_code, to make sure that
// any piece of code that is unused doesnt get a warning

#[allow(dead_code)]
fn unused_function() {}

fn main() {
    println!("Hello, world!");
}
