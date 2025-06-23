// We can change the type of numeric literals with a sufix

fn main() {
    let x = 1u8;
    let y = 2f32;
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&y));
}
