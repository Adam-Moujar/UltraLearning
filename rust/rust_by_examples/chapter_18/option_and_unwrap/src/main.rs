// Option is used when there's a possibility that we might not get a return value
// unwrap is used to get the value out of an Option or panic if it's none
// The adult has seen it all, and can handle any drink well.
// All drinks are handled explicitly using `match`.
#![allow(dead_code)]

fn give_adult(drink: Option<&str>) {
    // Specify a course of action for each case.
    match drink {
        Some("lemonade") => println!("Yuck! Too sugary."),
        Some(inner) => println!("{}? How nice.", inner),
        None => println!("No drink? Oh well."),
    }
}

// Others will `panic` before drinking sugary drinks.
// All drinks are handled implicitly using `unwrap`.
fn drink(drink: Option<&str>) {
    // `unwrap` returns a `panic` when it receives a `None`.
    let inside = drink.unwrap();
    if inside == "lemonade" {
        panic!("AAAaaaaa!!!!");
    }

    println!("I love {}s!!!!!", inside);
}

// We've covered the usage of ? for Results, you can also use it for Options
// It will return the underlying value if Some, otherwise will terminate whatever function and
// return None
fn next_birthday(current_age: Option<u8>) -> Option<String> {
    // If `current_age` is `None`, this returns `None`.
    // If `current_age` is `Some`, the inner `u8` value + 1
    // gets assigned to `next_age`
    let next_age: u8 = current_age? + 1;
    Some(format!("Next year I will be {}", next_age))
}

// Theres a way to make a certain pattern easier to write
// If we want to operate on a some to turn it into another some and a none into a none
// we can use map on option, e.g:
#[derive(Debug)]
enum Food {
    Apple,
    Carrot,
    Potato,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Peeled(Food);
#[allow(dead_code)]
#[derive(Debug)]
struct Chopped(Food);
#[allow(dead_code)]
#[derive(Debug)]
struct Cooked(Food);

// Peeling food. If there isn't any, then return `None`.
// Otherwise, return the peeled food.
#[allow(dead_code)]
fn peel(food: Option<Food>) -> Option<Peeled> {
    match food {
        Some(food) => Some(Peeled(food)),
        None => None,
    }
}

// Chopping food. If there isn't any, then return `None`.
// Otherwise, return the chopped food.
#[allow(dead_code)]
fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
    match peeled {
        Some(Peeled(food)) => Some(Chopped(food)),
        None => None,
    }
}

// Cooking food. Here, we showcase `map()` instead of `match` for case handling.
#[allow(dead_code)]
fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    // If Some(Chopped) -> Some(Cooked) else None -> None
    chopped.map(|Chopped(food)| Cooked(food))
}

// theres a problem, if map uses a function that returns an option, then we will get
// option<option<T>>.
// In these cases we use the function and_then, commonly known in other languages as flatmap.

#[derive(Debug)]
enum NewFood {
    CordonBleu,
    Steak,
    Sushi,
}
#[derive(Debug)]
enum Day {
    Monday,
    Tuesday,
    Wednesday,
}

// We don't have the ingredients to make Sushi.
fn have_ingredients(food: NewFood) -> Option<NewFood> {
    match food {
        NewFood::Sushi => None,
        _ => Some(food),
    }
}
// We have the recipe for everything except Cordon Bleu.
fn have_recipe(food: NewFood) -> Option<NewFood> {
    match food {
        NewFood::CordonBleu => None,
        _ => Some(food),
    }
}
// To make a dish, we need both the recipe and the ingredients.
// We can represent the logic with a chain of `match`es:
fn cookable_v1(food: NewFood) -> Option<NewFood> {
    match have_recipe(food) {
        None => None,
        Some(food) => have_ingredients(food),
    }
}

// This can conveniently be rewritten more compactly with `and_then()`:
fn cookable_v3(food: NewFood) -> Option<NewFood> {
    // The function we use return option so map would actually end up with
    // option<option<T>> so we use and_then for that to not happen
    have_recipe(food).and_then(have_ingredients)
}

// Theres some extra useful functions
// or
#[derive(Debug)]
enum Fruit {
    Apple,
    Orange,
    Banana,
    Kiwi,
    Lemon,
}

fn show_or() {
    let apple = Some(Fruit::Apple);
    let orange = Some(Fruit::Orange);
    let no_fruit: Option<Fruit> = None;

    // If none gets or, otherwise stays with the Some value
    let first_available_fruit = no_fruit.or(orange).or(apple);
    println!("first_available_fruit: {:?}", first_available_fruit);
}

// or else
fn show_or_else() {
    let apple = Some(Fruit::Apple);
    let orange = Some(Fruit::Orange);
    let no_fruit: Option<Fruit> = None;
    let get_kiwi_as_fallback = || {
        println!("Providing kiwi as fallback");
        Some(Fruit::Kiwi)
    };
    let get_lemon_as_fallback = || {
        println!("Providing lemon as fallback");
        Some(Fruit::Lemon)
    };

    // Difference is it takes a closure and its lazy
    let first_available_fruit = no_fruit
        .or_else(get_kiwi_as_fallback)
        .or_else(get_lemon_as_fallback);
    println!("first_available_fruit: {:?}", first_available_fruit);
}

// get_or_insert()
fn show_get_or_insert() {
    let mut my_fruit: Option<Fruit> = None;
    let apple = Fruit::Apple;
    let first_available_fruit = my_fruit.get_or_insert(apple);
    println!("first_available_fruit is: {:?}", first_available_fruit);
    println!("my_fruit is: {:?}", my_fruit);
    // If some gets the value else inserts into my_fruit the argument
}
//
// get_or_insert_with()
fn show_get_or_insert_with() {
    // Again the difference is that it takes a closure
    // and it evaluates lazily
    let mut my_fruit: Option<Fruit> = None;
    let get_lemon_as_fallback = || {
        println!("Providing lemon as fallback");
        Fruit::Lemon
    };
    let first_available_fruit = my_fruit.get_or_insert_with(get_lemon_as_fallback);
    println!("first_available_fruit is: {:?}", first_available_fruit);
    println!("my_fruit is: {:?}", my_fruit);
    // Providing lemon as fallback
    // first_available_fruit is: Lemon
    // my_fruit is: Some(Lemon)

    // If the Option has a value, it is left unchanged, and the closure is not invoked
    let mut my_apple = Some(Fruit::Apple);
    let should_be_apple = my_apple.get_or_insert_with(get_lemon_as_fallback);
    println!("should_be_apple is: {:?}", should_be_apple);
    println!("my_apple is unchanged: {:?}", my_apple);
}

fn main() {
    let water = Some("water");
    let lemonade = Some("lemonade");
    let void = None;

    give_adult(water);
    give_adult(lemonade);
    give_adult(void);

    let coffee = Some("coffee");
    let nothing = None;

    drink(coffee);
    drink(nothing);
}
