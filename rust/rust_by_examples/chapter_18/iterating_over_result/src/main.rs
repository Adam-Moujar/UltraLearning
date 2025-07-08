// Sometimes iter mapping through a list fails at some elements
// How do we handle it? well turns out in quite a couple of ways

fn main() {
    // How could it fail?
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings.into_iter().map(|s| s.parse::<i32>()).collect();
    // The first one is not a number so it will return error
    println!("Results: {:?}", numbers);

    // First way to handle it, skip errors using filter_map
    // and unwrap the values aswell
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("Results: {:?}", numbers);

    // maybe we actually care about the errors, we can actually also save the errors using map_err
    let strings = vec!["42", "tofu", "93", "999", "18"];
    let mut errors = vec![];
    let numbers: Vec<_> = strings
        .into_iter()
        .map(|s| s.parse::<u8>())
        .filter_map(|r| r.map_err(|e| errors.push(e)).ok())
        .collect();
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);

    // Maybe we dont want to fail at all, and if we do fail, the whole operation fails
    // Well collect does that for us alongside and explicit type annotation
    let strings = vec!["tofu", "93", "18"];
    // numbers has an extra result type annotation that helps collect turn it into a result
    let numbers: Result<Vec<_>, _> = strings.into_iter().map(|s| s.parse::<i32>()).collect();
    println!("Results: {:?}", numbers);

    // This is similar to the map_err but more elegant in my opinion
    // we split results and errors
    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);

    // But they are still wrapped in a result, if you dont like that, we can actually clean them up
    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}
