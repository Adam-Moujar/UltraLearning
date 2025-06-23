// We can create type conversions from custom types to other types

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

// You only need either from or into
// !!!! But !!!!
// If you implement into, you dont automatically get from,
// you do get into if you implement from tho

//impl Into<Number> for i32 {
//    fn into(self) -> Number {
//        Number { value: self }
//    }
//}
fn main() {
    let int = 5;
    let num: Number = int.into(); // you need the explicit type annotation otherwise the int doesnt
    // know what to turn into
    println!("My number is {:?}", num);
}
