// Closures, anonymous functions, yep.
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}
use std::thread;
fn main() {
    // This is a closure that takes a value and returns it
    // You dont need to type anotate it since the compiler will give the type of whatever value you
    // pass on it
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    // For example, after calling this closure on the String, the compiler will think that the
    // closure parameter is a string type
    //let n = example_closure(5); // So this is an error because you are trying to pass an integer
    //on a parameter which rust thinks should be a string.

    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    // Usually a closure will only borrow the variables it uses, mutable if the variable changes,
    // immutable otherwise, you can force a closure to own the variable it uses using the move
    // keyword
    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
    // println!("After defining closure: {list:?}"); // List cant be used anymore since the value
    // was moved to the closure
    //
    // Depending on what the closure does, an automatic trait will be applied to it.
    // or 2 of them, or all of them:
    // 1. FnOnce trait, applied to closures that can only be called once, so all closures are
    //    automatically FnOnce. If a closure moves captured values out, it will only implement
    //    FnOnce since it can only be called once.
    // 2. FnMut applied to closures that don't move captured values out of their body, but
    // *might* mutate captured values. These closures can be called more than once
    // 3. Fn applies to closures that dont move capture values out, dont mutate capture values or
    //    don't even capture values at all. These closures can be called more that once, and are
    //    pretty cool for concurrency since they dont change their environment
    //
    // If there's a closure, which is only called once, and doesnt capture values and mutates them.
    // It technically applies all three.
    //
    // Examples
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];
    let mut sort_operations = vec![];
    let value = String::from("closure called");

    list.sort_by_key(|r| r.width); // This is a fnmut. It doesnt move values outside
    //list.sort_by_key(|r| {
    //    sort_operations.push(value);
    //    r.width
    //});
    //This is an fnonce since it captures value, and then moves value to the sort_operation vector
    //and thus moves a captured value and therefore cannot be fnmut and only a fnonce. And sort by
    //key expects a fnmut closure
}
struct Rectangle {
    width: u32,
    height: u32,
}
