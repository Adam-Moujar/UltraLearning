// Arrays are quite similar to other langauges
// They save a type and they have length

// The new Rust addition to spice things up are slices
// Slices are ways to borrow a "slice" of an array, for iterating,
// for searching, for whatever reason you want the slice.
fn main() {
    let arr = [1, 2, 3, 4, 5];
    let arr_slice = &arr[1..3];

    println!("The array is: {:?}", arr);
    println!("The slice is: {:?}", arr_slice);
}
