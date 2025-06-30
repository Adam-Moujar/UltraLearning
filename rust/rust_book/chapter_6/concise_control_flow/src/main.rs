// There are common repeating patters in Rust that we can shorten
fn main() {
    // This is a pretty common pattern where we want to do something to
    // the data if and only if its exactly like one of the possibilities.
    let config = Some(3u8);
    match config {
        Some(max) => println!("The max is configured to be {max}"),
        _ => (),
    }

    // can be shortened to
    if let Some(max) = config {
        println!("The max is configued to be {max}");
    }
    // also has a else branch for the other cases we dont want
    if let Some(max) = config {
        println!("The max is configured to be {max}");
    } else {
        println!("No configured max");
    }

    // Another pattern that can be shortened
    //let state = if let Coin::Quarter(state) = coin {
    //    state
    //} else {
    //    return None;
    //};
    //to
    //    let Coin::Quarter(state) = coin else {
    //      return None;
    //  };
}
