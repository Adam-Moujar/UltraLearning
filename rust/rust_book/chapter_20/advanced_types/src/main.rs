// This is an extension of the topic we covered at the end of the last subchapter, advanced traits
// We discussed the NewType Pattern, which is just wrapping an old type in a wrapper class.
//
// We have covered one advantage of doing this, we can implement external traits on external types.
// Another advantage is that we can expose a public api to something that would be completly
// different than what it is normal

// We can also alias a type
type Kilometers = i32;

fn main() {
    // These 2 are the exact same
    let x: i32 = 5;
    let y: Kilometers = 5;

    // where aliasing is useful is avoiding long types like:
    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

    fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
        // --snip--
    }

    fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
        todo!()
    }
    // using aliasing we can shorten it to:
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_aliased_type(f: Thunk) {
        // --snip--
    }

    fn returns_aliased_type() -> Thunk {
        todo!()
    }

    // rust has a special type that never returns
    fn bar() -> ! {
        // --snip--
        panic!("We cant return anything from this function, even ()")
    }

    // Dynamic sized stypes and sized trait
    // Some types have different sizes which vary at runtime like:
    let s1: &str = "Hello there!";
    let s2: &str = "How's it going?";
    // There 2 strs would have a size of 12 and 15 respectively.
    // And so when rust assigns a size to the type what size does it give?
    // We cant, we use run time allocated memory, heap memory and store the pointer in the stack
    // which is why string slices are &str and not str,
    // str could technically work with any dynamic pointer, Box<str> or Rc<str> would work.
    //
    // There's a trait specific for types whose size is known at run time, Sized.
    // This is automatically implemented for anything whose size is known so we dont generally need
    // to worry about it
}
