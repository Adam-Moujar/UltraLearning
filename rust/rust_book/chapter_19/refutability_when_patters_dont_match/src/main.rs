// Some patters can fail, we call those refutable
// for example
fn main() {
    let x: Option<i32> = None;

    // let Some(x) = x; // Pattern does not fit
    let Some(x) = x else {
        return;
    };
    // Pattern fits since we covered the else situation
    //
    let x = 5 else {
        return;
    };
    // This is a warning since let x = * is a irrefutable pattern
    // so the else clause will never run
}
