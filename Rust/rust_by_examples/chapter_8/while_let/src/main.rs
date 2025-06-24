// keep doing something while the pattern holds

fn main() {
    let mut optional = Some(0);

    // as long as optional looks like Some(i) the loop runs
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{i:?}`. Not too big");
            optional = Some(i + 1);
        }
    }
}
