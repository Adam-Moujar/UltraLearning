// Variadic interfaces take an arbitrary number for example println!
// We already saw how using recursive macro calls we can create something similar
//
macro_rules! calculate {
    (eval $e:expr) => {
        let val: usize = $e;
        println!("{} = {}", stringify!($e), val);
    };

    (eval $e:expr, $(eval $es:expr),+) => {
        calculate! { eval $e }
        calculate! { $(eval $es),+}
    };
}
fn main() {
    calculate! { // Look ma! Variadic `calculate!`!
        eval 1 + 2,
        eval 3 + 4,
        eval (2 * 3) + 1
    }
}
