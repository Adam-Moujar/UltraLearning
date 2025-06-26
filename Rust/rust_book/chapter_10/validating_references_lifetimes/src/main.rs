// One thing we have been hiding is lifetime, how long a reference is valid.
// For most of the time, its implicit, its only necessary when the lifetime of the
// reference is ambigious and the compiler needs you to decide which of the lifetimes to choose.
//
// This is probably the most unfamiliar concept so far.
//
// The main aim is to prevent dangling references
//

// Lets see why explicit lifetimes are necessary
// This is a simple function which takes 2 string slices and returns which one is larger
// However without lifetimes, the borrow checker doesnt work if the returned lifetime should have
// the same lifetime as x or y, infact, its not definitely not obvious since it could have both
//
// Lifetimes simply try to describe the relationship of multiple references.
// Here we declare a lifetime 'a which all references share.
// We are saying that for lifetime 'a, both references must live atleast (yes it's atleast
// since for references, being contained in 'a is fine, the problem arises when you go outside
// this lifetime) 'a and so does the return value.
// This doesn't change the lifetime of the function, but rather rejects anything
// that would produce a different life time than this
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    //let r;
    //{
    //    let x = 4;
    //    r = &x;
    //}
    //println!("r: {r}");
    //
    //This would cause a dangling reference since r would point to a dropped value x
    //
    // The rust compiler uses a borrow checker to know when a dangling reference happens
    // If a reference ever points to something with a lifetime smaller that its own lifetime, bad
    // things happen
    //
    // This is okay because 'a is the inner scope and string1, string2 and result all live until
    // atleast the inner scope
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }
    // This is not okay because 'a is the outer scope because result is outside and string2  does
    // not live until the outer scope ends. So cant compile
    //let string1 = String::from("long string is long");
    //let result;
    //{
    //    let string2 = String::from("xyz");
    //    result = longest(string1.as_str(), string2.as_str());
    //    println!("The longest string is {result}");
    //}
}

// Explicit timelines are a bit tricky, if you have 2 references as parameters only need to return the first paramenter
// Then you only need to annotate the first parameter with a lifetime and the return, since the
// second parameter has no bearing on the lifetime of the return. e.g
fn new_longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// Functions is not the only case where we need to worry about lifetimes
// In structs, most of the time we wan't the struct to actually own its data,
// but sometimes, it might be that we want the struct to, as a field, have a reference to something
// in that case, the struct needs to have a lifetime for the field, e.g

struct StructLifetime<'a> {
    part: &'a str,
}
// This will have the lifetime of whatever we reference
//
// Impl structs with lifetimes look like
impl<'a> StructLifetime<'a> {
    fn level(&self) -> i32 {
        3
    }
}

// Special lifetime: 'static
// There's a special lifetime, 'static, which lives for the duration of the entire program. It
// always liveees, we can always just slap a 'static lifetime and call it a day, but thats hacky
// and bad practice and you should think twice before doing it.
//
// Using everything we have learned so far
// Behold, an abomination of a function
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}
