// Match guards are extra conditions/filters on the arm

enum Temperature {
    Celsius(i32),
    Fahrenheit(i32),
}

fn main() {
    let temperature = Temperature::Celsius(5);

    match temperature {
        //boom match guard, pretty cool
        Temperature::Celsius(t) if t > 30 => println!("{t}C is above 30 celcius"),
        Temperature::Celsius(t) => println!("{t}C is equal to or below 30 celcius"),
        Temperature::Fahrenheit(t) if t > 86 => println!("{t}F is above 86 fahrenheit"),
        Temperature::Fahrenheit(t) => println!("{t}F is equal to or below 86 fahrenheit"),
    }
}
