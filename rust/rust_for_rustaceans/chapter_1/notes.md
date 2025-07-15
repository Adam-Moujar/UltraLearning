## Notes

This chapter, and the whole book really, is a bit of a more in-depth look at the concepts we have learned in the Rust book. 
It serves both as a recap if we needed it, and as a way to get more intimate with the way the things presented.
This Chapter 1 - Foundation, tackles memory, that is:

- Heap vs Stack
- Primitives
- Static memory
- Ownership
- Borrowing and lifetimes
- References

Let's begin with some memory terminology: 

### Memory

Lets first clarify what exactly we mean when we say value. This was something repeated a lot in the rust book, but what does it exactly
mean? A value is a type combined with an element of that type's set of values. Or in less complicated terms, value is the thing assigned to 
a type. A value is stored in a place, a location in memory that can hold a value, this can be the heap, the stack, registers, who knows.
Most commonly however, they are stored in variables. There's a special type of value which is a pointer, it holds the address of another region
in memory, we can dereference the pointer to look into the address and even have multiple pointers poiting to the same address. 

Consider:

```
let x = 42;
let y = 43;
let var1 = &x;
let mut var2 = &x;
var2 = &y;

```

We have the values: 42, 43 as i32, the address of x and the address of y as pointers. There are four variables, x, y, var1, var2.
The former store the numeric values while the latter store the pointer values. Interestingly, var1 and var2 initially hold the same value,
but they are independed copies of that value. We see this later when we change var2 to store the address of y, var1 doesnt change. 
The `=` operator stores the right hand side expression in the location expressed in the left hand side. For example, consider:

`let string = "Hello world";`

The actual value stored in string is not `"Hello world"` but rather a pointer to the first character of the string. But in that case,
where is the string stored? 

### Variables in Depth

The definition of a variable we gave in depth is too vague to be useful. There are other models we can use to help reason about what we
are writing, the book will cover 2 categories: 

- High-level model: More useful when thinking about borrows and lifetimes.
- Low-level model: More useful when thinking about unsafe code and raw pointers.

#### High-Level Model

In this model, variable aren't places that hold bytes, but rather names given to values as they are used in the program. 
When a variable is accessed, we can imagine drawing a line from the previous access of the variable to the new access. 
This establisheds a dependency relation between the accesses. 

In this model, a variable only exists so long as it holds a legal value and if the value is moved, no lines can be drawn 
from the variable anymore. These lines are often called flows, each one traces a lifetime of a particular instance of a value. 
These flows can fork and merge and the program that check that at any given point, all flows that exist in parallel are compatible. 
That is for example:

- No 2 parallel flows that with mutable access to a value.
- No flow that borrow a value when there isn't a flow that owns the value.
- etc...

Let's look at some examples:

```
let mut x;
// this access would be illegal, nowhere to draw the flow from:
// assert_eq!(x, 42);
x = 42;
// this is okay, can draw a flow from the value assigned above:
let y = &x;
// this establishes a second, mutable flow from x:
x = 43;
// this continues the flow from y, which in turn draws from x.
// but that flow conflicts with the assignment to x!
assert_eq!(*y, 42);
```

The borrow checker will reject this snippet since the have a <em>exclusive</em> (its just marked as &mut since the variable 
has the mut keyword) flow. Then a <em>shared</em> (an immutable & since y doesn't have mut keyword) flow. We use the exclusive
flow to change the value and the shared flow to read the value which is not allowed in Rust (we can't have a &mut and & at the same time)
and so it is rejected.

This model matches how the compiler and borrow checker reason about your program and is actually used to produce efficient code by the 
compiler

#### Low-Level Model

In this model, variables are names for memory locations. When you assign a value to the memory slot, it is filled, the old value (if it had
one) is dropped and replaced. When you access a memory location using a variable, the compiler checks that it isn't empty (the value hasn't 
been initialised or its value has been moved). 

Thats the model, the reason for the lack of depth in the explanation is that this is the default model that comes with languages like
C/C++. Low level languages tend themselves better to this model and thus it is likely that you are more comfortable with this model. 

Now that we know the 2 different models for memory, lets look at what memory actually is: 

### Memory Regions

There are different regions of memory so we will cover the 3 most important ones that you will probably use in every Rust program.

#### The Stack

The stack is the space that the program uses as scratch space (temporary memory area) for functions. Any time a function called a 
frame is added to the top of the stack. During runtime, if we take a look at the very bottom of a stack we would see the frame of the
main function for example. The frame contains all the variables within that function, along with arguments. When a function returns, the 
stack frame of the function is reclaimed.

This is a biiiig hint to the idea of lifetimes. Any variable stored in a frame on the stack cannot be accessed after the frame goes away. 
So any reference to the variable must have a lifetime at most as long as the lifetime of the frame.

#### The Heap

The heap stands in opposition to the stack, though they both are still just memory locations in RAM. Values in heap memory will live until 
explicitly deallocated. They are most useful for example when we want a value to live beyond the lifetime of the current function frame. 

It is also useful when allocating memory during runtime, when you do so, you get a pointer to the start of the memory block you allocated.
And you can continue to use it until you deallocate it or freeing it.

Again to hint to the lifetime idea, a heap allocated pointer lives for as long as the program keeps it alive, it is not tied to a function's
lifetime.

The primary way we create heap allocated memory in rust is through the usage of `Box::new(value)`. The value is placed on the heap and we 
are given a pointer to it. If you allocate memory in heap and don't deallocate it, it will live forever until the program ends. This is called
leaking memory and it's something you want to avoid, though sometimes we will purposely leak memory if we want to keep it alive until the end
of the program. In these cases we use `Box::leak` to specify that it should be `'static`.

#### Static Memory

Its a catch-all term for closely related regions in the file the program is compiled into. These regions are loaded into the program's memory 
when it is executed. The values in static memory live for the entire duration of the program.

The most common way we will see this be used in Rust is through the usage of `'static` lifetimes. Things that have a static lifetime are 
essentially self-sufficient, they will stick around until the end of the program or we get rid of them explicitly.


### Ownership

Rust's memory model central idea is that every value has a single owner, a location (usually a scope) that is responsible for dealloaction
each value. If the value is moved, the ownership of the value goes from the old location to the new one. In which case you are not allowed
to access the value using flows (the thing we discussed up above) from the old location.

There are some types that don't follow this rule, type's that implement the Copy trait do not movie but rather the value is copied and 
both the new and old locations remain accessible. Most primitive types in Rust are Copy. We can make a custom type into a Copy type but it
must be possible to duplicate the type's values by only copying their bits. So cannot have any non-Copy type and any type that owns a resource
that needs to be deallocated. For example:

```
let x1 = 42;
let y1 = Box::new(84);
{ // starts a new scope
let z = (x1, y1);
// z goes out of scope, and is dropped;
// it in turn drops the values from x1 and y1
}
// x1's value is Copy, so it was not moved into z
let x2 = x1;
// y1's value is not Copy, so it was moved into z
// let y2 = y1;
````
Let y2 = y1 at the end would return an error because the value in y1 was moved into z which got dropped and so we cant use the old flow anymore.
However x2 = x1 would compile because x1 is a Copy type and so when we assigned x1 to z, the value was copied rather than moved and we 
could still use the old flow.

### Borrowing and Lifetimes

Rust allows the owner of a value to lend the value to others through references. Pointers include additional information about how they can
be used, whether they provide exclusive access to the referenced value, or can the value also have other references aswell.

#### Shared References

A shared refrence, `&T`, is a pointer that may be shared (duh). Any number of shared references may exist and each reference is a Copy type so
we can trivially make more of them. Values behind shared references are not mutable nor can you cast it to a mutable one.

The Rust compiler will assume that the value behind a shared reference <em>will not change</em>.

#### Mutable References

The alternative to a shared reference is a mutable reference: `&mut T`. This is more restrictive, its an exclusive reference, you can only have 1
mutable reference and no other references at all. 

The difference between a mutable reference and an owner is that the owner is reponsible for dropping the value. 

#### Interior Mutability

These are special types that can allow you to mutate values even with shared references. There are 2 categories of types that provide interior
mutability are: 

- Types like `Mutex` and `RefCell` which containt safety mechanisms that make sure there's only 1 mutable reference at a time. 
- The other type are types that don't provide mutable references to the type, but rather give you methods to manipulate the valyes in place. 
Stuff like `std::sync::Atomic` for example and `std::cell::Cell`. 

#### Lifetimes

A `lifetime` is just a region of code that a reference must be valid for. It frequently consides with scope, but they are not they do not
have to be the same.

#### Borrow Checker

At the heart of Rust and its lifetimes is the `borrow checker`. Whenever a reference with a lifetime `'a` is used, the borrow checker
checks that `'a` is still alive. It goes back to where `'a` begins and checks that no conflicting use in its path. This makes sure
that the reference points to a value that is safe to access.
