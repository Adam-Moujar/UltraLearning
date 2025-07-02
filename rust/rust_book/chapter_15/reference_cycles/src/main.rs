// Rust's rules for memory are pretty great, restrictive, but great.
// They make sure that you do not engage in unsafe code.
// But it is not perfect, reference cycles, using Rc and RefCell make it possible for us
// to create memory leaks. And it's not foreign to do something similar in recursive data types

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
    // println!("a next item = {:?}", a.tail());
    // This will crash the program. The list data type loops. a points to 5 which points to 10
    // which points back to 5 and so it will look forever and crash.
    // We can fix this using a Weak<T> which doesnt increase the strong count.
    // This works by making sure that whenever you want to uses Weak <T> you need to first
    // make sure that the value you will use still exists.

    // This is a good example of when to use Weak<T>
    // We have a node, like a node in a binary tree
    // except these nodes also want to know who their parent are
    // Problem, if you simply add a parent node, that creates a cycle.
    // Big no no. Instead we will make them a weak node, this makes it
    // so that a Parent owns their children, but a children doesnt own their parents
    // so its not a case of cycle anymore.
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }
}
