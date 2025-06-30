struct Foo {
    x: (u32, u32),
    y: u32,
}
fn main() {
    let foo = Foo { x: (2, 2), y: 2 };

    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {b}, y = {y}"),
        //order of structs elements is not fixed
        Foo { y: 2, x: i } => println!("y is 2, i = {i:?}"),
        _ => todo!(),
    }
}
