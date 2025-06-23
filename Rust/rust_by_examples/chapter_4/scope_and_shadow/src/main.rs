// Again we've talked about scope, which is what variables are allowed to live to.
// Shadowing is creating a new variable under an old name, shadowing the first variable.
fn main() {
    let scope = 1;
    {
        let scope = 2; // shadowed the first scope var
        println!("scope is: {scope}");
    }; // second scope goes out of scope here
    println!("scope is: {scope}");
}
