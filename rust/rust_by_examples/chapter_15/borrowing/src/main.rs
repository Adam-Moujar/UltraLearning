// Borrows, I hope by now you know what they are, why they exist, why they are useful.
//
// The most interesting part here is the talk about the ref pattern.
#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let c = 'Q';

    // These are the same
    let ref ref_c1 = c;
    let ref_c2 = &c;

    let point = Point { x: 0, y: 0 };
    // The main usefulness of a ref is in pattersn, or borrowing the inner parts of multi-data type
    let _copy_of_x = {
        // `ref_to_x` is a reference to the `x` field of `point`.
        let Point {
            x: ref ref_to_x,
            y: _,
        } = point;

        // Return a copy of the `x` field of `point`.
        *ref_to_x
    };
}
