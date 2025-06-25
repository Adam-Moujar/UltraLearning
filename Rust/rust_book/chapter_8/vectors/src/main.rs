// Vectors is the collection type you'll probably use the most and the first one we will learn about
fn main() {
    let mut v = vec![1, 2, 3]; // A pretty useful macro to create a Vector

    v.push(4);
    v.push(5);
    v.push(6);

    let v = v;

    let third: &i32 = &v[2]; // we dont want to take ownership, so ref
    println!("The third elemt is: {third}");

    let third: Option<&i32> = v.get(2); // Another way to get third with more edge cases
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    for i in v.iter() {
        // iterating baby
        println!("{i}");
    }
}
