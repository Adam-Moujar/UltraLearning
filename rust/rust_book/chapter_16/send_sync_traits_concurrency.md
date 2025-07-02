## Notes

Note that all the concurrency features (i think all, if not nearly all) we have covered
are part of the standard library (std), not the langauge.
Which technically means, I could write my own concurrency features.
We have 2 traits to help us with that:

- Send, trait that indicates that ownership of values can be transferred between threads.
- Sync, trait that indicates that it is safe for values to be references from multiple threads.

Nearly all types implement send and sync. At the very least, every primitive type implements these 2 traits.
And types which compose of only primitives or types that implement send or sync also implement send or sync. 

Rc is an except to this, it does not implement Send or Sync which is why we had to use Arc.

Manually implementing send and sync is not safe. Which is why the compiler does it for you, as long as the types 
you compose use send and sync types, they will also be send and sync.

