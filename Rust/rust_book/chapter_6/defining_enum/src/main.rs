// Enums are kinda similar to other languages, except they can also contain values.
// Enums are also used in match cases, kind of like switch cases

// Normal ip addr in other languages
enum IpAddrKind {
    V4,
    V6,
}

// Enum with values passed
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    // Example of using match for enums
    // Its quite helpful, love this feature.
    match home {
        IpAddr::V4(a, b, c, d) => println!("Ip addr v4"),
        IpAddr::V6(s) => println!("Ip addr v6"),
    }
}
