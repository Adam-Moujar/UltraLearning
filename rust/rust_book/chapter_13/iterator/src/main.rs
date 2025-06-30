// I am pretty familiar with iterators due to scala and c++
// In general, to go from list to iter use into_iter()
// to go from iter to list you need collect()
// You can use iterator adapters use iterators and take closures as parameters to do cool stuff
// like sum, filter, map etc...
fn main() {
    println!("Hello, world!");
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}
