// vectors are re-sizable arrays. They are also slower than arrays.
// I am pretty sure they actual array is heap allocated and the vector is just a pointer to the
// array. When they array is full and you need to add an element, a new bigger array is
// constructed, memory moved from old array to the new array, the old array is destroyed and the
// pointer is updated to the new array. This is why its slower than arrays
fn main() {
    let mut collected_iterator: Vec<i32> = (0..10).collect();
    collected_iterator.push(0);
}
