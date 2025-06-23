// This will be a testcase, kind of like a case study of a common way to use enums

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        List::Nil
    }

    // very interesting case where we function does want to take ownership of the object
    // since we want the new returned list to contain self, the old list should be discarded
    fn prepend(self, elem: u32) -> List {
        List::Cons(elem, Box::new(self))
    }

    // recursive len call
    fn len(&self) -> u32 {
        match *self {
            // again, very interesting case where the tail needs to be a ref since we only borrow
            // self, so we again only need to borrow tail.
            // though after rust 2018, we can remove the derefence of self, and also remove the ref
            // of tail
            List::Cons(_, ref tail) => 1 + tail.len(),
            List::Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            List::Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            }
            List::Nil => {
                format!("Nil")
            }
        }
    }
}
fn main() {
    // Create an empty linked list
    let mut list = List::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
