// What the hell is a hashset? A hashmap where we only care about the keys
// Well what is the point of it? just store the keys in a vector
// The difference is that hashset has no duplicates
//
// Sets also have 4 primary operations:
//
// Union: given 2 sets, get the unique elements in both sets
// difference: gets the elements that are in the first and not the second,
// intersection: get the elements in both
// symmetric_difference: gets the elements that are in either one set, but not both
//
// And hold on wait, this is just venn diagrams
use std::collections::HashSet;
fn main() {
    let mut a: HashSet<i32> = vec![1i32, 2, 3].into_iter().collect();
    let mut b: HashSet<i32> = vec![2i32, 3, 4].into_iter().collect();

    assert!(a.insert(4));
    assert!(a.contains(&4));

    // `HashSet::insert()` returns false if
    // there was a value already present.
    let test = b.insert(4);
    println!("What is test?: {test}");

    b.insert(5);
    println!("A: {:?}", a);
    println!("B: {:?}", b);

    // Print [1, 2, 3, 4, 5] in arbitrary order
    println!("Union: {:?}", a.union(&b).collect::<Vec<&i32>>());

    // This should print [1]
    println!("Difference: {:?}", a.difference(&b).collect::<Vec<&i32>>());

    // Print [2, 3, 4] in arbitrary order.
    println!(
        "Intersection: {:?}",
        a.intersection(&b).collect::<Vec<&i32>>()
    );

    // Print [1, 5]
    println!(
        "Symmetric Difference: {:?}",
        a.symmetric_difference(&b).collect::<Vec<&i32>>()
    );
}
