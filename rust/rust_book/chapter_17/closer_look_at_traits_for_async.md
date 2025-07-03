## Notes

Lets take a closer look at all the traits we have used so far

Starting with arguably the most important, the Future trait:
Here's how it is defined:

```
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

Yep thats it!

The poll type is defined like:

```
enum Poll<T> {
    Ready(T), // got the value, stored in T
    Pending, // still getting the value, check later
}
```

### Pin and unpin traits

Kind of a bit like &mut and Box and Rc, its a pointer-like type.
Now its pointer-like in the sense that its a tool the compiler uses to enforce constraints on
pointer usage, not that it functions like a pointer

In general what pin does is that makes sure the underlying type cannot move in memory. It pins
it to the memory, thats why its give that name. The reason why is that types that are
self-referential run into a problem. If they move in memory, then you have a problem that the
reference pointed to the old location in memory, which is now invalid. Pin makes sure that it
cannot move.

Now heres a good question, what happens if we pin a type which is not self-referential?
A pin is used to tell the compiler that the type is self-referential so moving is a big no-no,
but in the case that it is not referential, what then?

Thats where Unpin comes in, its a trait automatically applied by the compiler to say that a type
is not self referential and is safe to unpin

### Stream trait

As we learned earlier, stream is similar to asynchrous iterators. Stream has no definition in the
standard library, but it comes packaged with many of the common future crates in Rust. 
Stream takes the idea of iterators, packaged together with Future and the poll method,
the definition of Stream looks like:

```
use std::pin::Pin;
use std::task::{Context, Poll};

trait Stream {
    type Item;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>
    ) -> Poll<Option<Self::Item>>;
}
```

Just like iterator next, we get an option of the item, and just like future, it returns a poll.
