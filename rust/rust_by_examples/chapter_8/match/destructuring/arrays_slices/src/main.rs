// Arrays and slices can also be destructured in much of the same way as tuples

fn main() {
    let array = [1, -2, 6];

    match array {
        [0, second, third] => {
            println!("The first value is zero, the second is {second}, and third is {third}")
        }
        [-1, _, _] => println!("We are ignoring the second and third values"),
        // This one is quite cool, we can give the skipped section a name and use it as a slice
        [first, middle @ .., last] => {
            println!("array[0] = {first}, middle = {middle:?}, array[-1] = {last}")
        }
    }
}
