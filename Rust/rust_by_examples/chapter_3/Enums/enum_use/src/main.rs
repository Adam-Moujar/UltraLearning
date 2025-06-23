// Just enum stuff

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    // c Struct style enum
    Click { x: i64, y: i64 },
}

type we = WebEvent;
fn main() {
    let x = we::Click { x: -10, y: 10 };
    match x {
        WebEvent::Click { x, y } => println!("Click at: {x},{y}"),
        _ => println!("bye"),
    }
}
