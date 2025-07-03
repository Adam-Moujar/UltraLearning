## Notes

There is no consensus in what features makes a language object oriented. Rust has influence from many 
paradigms, functional and OOP being some of them. Here's some of the ways Rust supports common characteristics
of OOP. 

### Objects Contain Data and Behaviour

The book Design Patterns: Elements of Reusable Object-Oriented Software by Erich Gamma, Richard Helm, 
Ralph Johnson, and John Vlissides (Addison-Wesley, 1994), defines OOP:

```
Object-oriented programs are made up of objects. An object packages both data and the procedures 
that operate on that data. The procedures are typically called methods or operations.

```
If this is the definition, Rust is object oriented, structs and enum have data and impl blocks 
provide methods on structs and enum. 

### Encapsulation that Hides Implementation Details

Encapsulation is allow about hiding implementation details, which again Rust ticks these boxes using
the keyword pub to decide what implementation is public or not.

### Inheritance as a Type System and Code Sharing

Inheritance is where objects can inherit element from another objects definition. 
If this is a requirement for a language to fall under OOP, then Rust is NOT one. 

However, inheritance is largely used for 2 reasons: 

1. Sharing code, so that a child class/object shares overarching functionality, this can be 
done in Rust using traits and impl blocks.

2. Polymorphism, an object can substitude for another object, usually a child substituting for a
parent object during runtime. Again this can be implemented in Rust using traits and using impl Trait
as a parameter for a function. 


