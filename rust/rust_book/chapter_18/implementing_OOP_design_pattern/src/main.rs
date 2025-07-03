// State pattern is a OOP design pattern. We define a set of states a value can have.
// And the value's behaviour changes depending on state.
//
// The advantage of using state pattern is that when the requirements of the program change.
// We only need to update code inside the value to change its rule, or perhaps add more states.
// Just like with the last section, we will implement something to showcase how it works,
// in this case, we will be implementing a blog post which has multiple states

pub struct Post {
    state: Option<Box<dyn State>>, // the state
    content: String,               // the content
}

impl Post {
    // the post starting state is draft
    // and it has no content
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    // adding content to the field
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // this is a getter for the content
    // if the state is draft or pending review, it shows nothing
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    // upgrade from draft to pending review
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    // upgrade from pending review to publish
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

// All the different states of the post and their implementation
// Draft, the starting state
struct Draft {}

impl State for Draft {
    // if someone requests a review, the state is upgraded to pending review
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    // approve does nothing on draft
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // theres a default implementation of content
    // in this case, draft has no content
}

// After draft, still hasnt been approved
struct PendingReview {}

impl State for PendingReview {
    // request review has already been called, we are pending approval
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    //if approved we can finally be published
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

// The final state where the blog post is shown
struct Published {}

impl State for Published {
    // we have already reviewed and approved
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

// Now to be fair, this is a lot of work for something we can do using Enums
// There are pros to and cons to state patterns:
//
// Pros:
//
// - State functionality is in a single place
// - We don't need to litter the program with match expression with many different arms
// - State patterns are easy to extends and add functionality and states
//
// Cons:
// - There's coupling between states, if we were to add a state between pending review and
// approval we would need to go back and change the code to match that
//
// - We have duplicated some logic, which does nothing and returns self and the approve and request
// review
//
// -
fn main() {
    println!("Hello, world!");
}
