// Let can bind variables to values if the pattern matches, else diverges if the pattern doesnt match
// Else usually breaks, or returns errors, or panics!
use std::str::FromStr;

fn get_count_item(s: &str) -> (u64, &str) {
    let mut it = s.split(' ');
    // We match a pattern in this case (Some(var), Some(var)) with something (it.next(), it.next())
    // if the pattern matches, the variables get binded with the value
    // if it doesnt the else runs, usually handling the error
    let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
        panic!("Can't segment count item pair: '{s}");
    };
    let Ok(count) = u64::from_str(count_str) else {
        panic!("Can't parse integer: '{count_str}'");
    };
    (count, item)
}
fn main() {
    assert_eq!(get_count_item("3 chairs"), (3, "chairs"));
}
