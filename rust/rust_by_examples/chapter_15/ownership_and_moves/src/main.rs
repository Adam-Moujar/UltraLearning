// Since variables are in charge of freeing their resources, each
// resource can only have one owner, otherwise you get the problem of freeing things more than once
//
// We can do something interesting, partial move, where we move only one thing in data which was
// multiple things.
//
// Things that implement drop cant do this, since when it drops, the whole thing will be used up.
fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    //impl Drop for Person {
    //    fn drop(&mut self) {
    //        println!("Dropping the person struct {:?}", self)
    //    }
    //}

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // Partially moving name, age is using ref so its a borrow not a move.
    let Person { name, ref age } = person;
    // Error! borrow of partially moved value: `person` partial move occurs
    //println!("The person struct is {:?}", person);

    // `person` cannot be used but `person.age` can be used as it is not moved
    println!("The person's age from person struct is {}", person.age);
}
