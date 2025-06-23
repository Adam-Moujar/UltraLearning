// This one is also quite simple but cool since its a side effect of
// being able to shadow a variable.
// We can freeze a variable by shadowing a mutable variable as immutable
fn main() {
    let mut mutable_variable = 7i32;
    {
        let mutable_variable = mutable_variable;
        //mutable variable is frozen now and its immutable

        // mutable_variable = 10; // doesn't work since it's immutable
    }

    mutable_variable = 3; //ok, back to mutable
}
