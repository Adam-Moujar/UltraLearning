// For in interates through an iterator.
// We can create a iterator using the range notion a..b, yields an iterator from a, to b-1.

fn main() {
    // we could use for n in 1..=100 if we want it to be inclusive
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{n}");
        }
    }

    // we have ways to convert collections to iterators.
    // By default using for in, on a collection automatically used into_iter to convert the
    // collection into an iterator.
    // There are 3 ways to convert:
    // into_iter, consumes the collection so that the collection cant be available after loop.
    // iter, borrows each element leaving the collection fine to be used
    // iter_mut, mutably borrows each element of the collection.
}
