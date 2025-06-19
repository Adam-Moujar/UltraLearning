fn main() {
    // The {} will be replaced with the arguments
    // In this case it will be replaced with 31
    println!("{} days", 31);

    // You can use positional arguments to specify what argument the {} gets replaced by
    // In this:
    // "Alice this is Bob. Bob this is Alice" will be printed
    println!("{0}, this is {1}. {1}, this is {0}.", "Alice", "Bob");

    // You can even name the arguments and use those names to specify what gets replaced
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // We also have different formats for different number bases
    println!("Base 10              : {}", 69420);
    println!("Base 2  (binary)     : {}", 69420);
    println!("Base 8  (octal)      : {}", 69420);
    println!("Base 16 (hexadecimal): {}", 69420);

    // We can prettify the text, for example
    // To right justify the text with a specific width:

    println!("{number:>5}", number = 1);

    // To pad numbers with extra zeroes,

    println!("{number:0>5}", number = 1);

    // Instead of right, we can left adjust by flipping the >

    println!("{number:0<5}", number = 1);

    // If you want to use variables as the format specifier using $

    println!("{number:0>width$}", number = 1, width = 5);

    // Obviously you cant use more arguments than you have available

    // println!("My name is {0}, {1} {0}", "Bond");

    // That would be an error
    //
    // Only types that use the fmt::Display can be printed using {}

    #[allow(dead_code)]
    struct Structure(i32);

    // println!("This struct {} won't print", Structure(3));

    // That comment wouldn't work since struct does not implement Display

    // Last thing, you can directly use variables as your format specificer and argument name

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
}
