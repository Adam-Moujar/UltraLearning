// So far, we have seen how we can introduce concurrency into our code using threads.
// However threads aren't the only way to make concurrent programs.
// We have async, a way to tell the program to do something at the same
// time as another task.
//
// Now there is a subtle difference between concurrency and parallelism.
// Concurrency is working on different tasks before any of them are complete,
// often requiring to switch between them.
//
// But parallelism is splitting the tasks and having a thread take a task and work on it
// alone.

// For a cpu with only a single core, it can still work concurrently, switching back
// and forth between tasks and "multi-tasking" but it can't work in parallel. Except if it
// has multiple cores, then each core can work on a task and achieve actual parallel work.
//
// For async, rust has something called the future trait, types that implement this trait
// are all about values that are not ready yet but will become ready at some point in the future.
// You can use async to let the cpu know that this task can be done concurrently, and use await if
// you need to wait for the future value to become ready.
//
use trpl::{Either, Html}; // trlp is a crate, short for the rust programming langauge, bundled by the
// official rust developers which holds a lot of useful crates.
fn main() {
    let args: Vec<String> = std::env::args().collect();
    // We cant main async, and to run async code we need to use await, but we can only use await if
    // we are in an async context. This is quite a conundrum.
    // Asyncs need a runtime, which main doesnt provide by itself, and trlp::run provides a runtime
    // for the async to use.
    trpl::run(async {
        let title_fut_1 = page_title(&args[1]);
        let title_fut_2 = page_title(&args[2]);
        let (url, maybe_title) = match trpl::race(title_fut_1, title_fut_2).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };

        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("It's a page title is: `{title}`"),
            None => println!("It's title could not be parsed."),
        }
    })
    // Each await point is a place where control is handed to runtime.
    // Rust needs to keep track of the state involved in an async block, whether it is waiting,
    // whether it can run code, to know when to move to something else and when to come back
    // Rust has an invisible state machine that saves the state at each await point
    //
    // Now something has to execute this state machine, and that is the runtime, specifically the
    // executor of the runtime.
    //
    // The reason why main cant be async, because then we would need something to manage the state
    // of main, but main is the starting point of the program. So we needed async::run to start the
    // runtime so we can use async.
}

// Making a function async
async fn page_title(url: &str) -> (&str, Option<String>) {
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html());
    (url, title)
}
