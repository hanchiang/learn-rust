// Trade-offs of the State Pattern:
// Advantages
// 1. Encapsulation of Post  behaviours in each state
// 2. The way we organized the code, we have to look in only one place to know the
// different ways a published post can behave: the implementation of the State trait on the Published struct.
// The implementation using the state pattern is easy to extend to add more functionality.

// Disadvantages
// 1. Because the states implement the transitions between states, some of the states are coupled to each other.
// If we add another state between PendingReview and Published, such as Scheduled,
// we would have to change the code in PendingReview to transition to Scheduled instead
// 2. Duplicated logic. If we had a lot of methods on Post that followed this pattern, we might consider defining a macro to eliminate the repetition.

// See code section starting from PostImproved below for an improved implementation

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self,  text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        // Because the goal is to keep all these rules inside the structs that implement State,
        // We call a content method on the value in state and pass the post instance (that is, self) as an argument.
        // when we call content on the &Box<dyn State>, deref coercion will take effect on the & and the Box so
        // the content method will ultimately be called on the type that implements the State trait.
        self.state.as_ref().unwrap().content(self)
    }

    // Advantages of the state pattern: the request_review method on Post is the same no matter its state value.
    // Each state is responsible for its own rules.
    pub fn request_review(&mut self) {
        //  We call the take method to take the Some value out of the state field and leave a None in its place,
        // because Rust doesn’t let us have unpopulated fields in structs
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
}

trait State {
    // Note that rather than having self, &self, or &mut self as the first parameter of the method,
    // we have self: Box<Self>. This syntax means the method is only valid when called on a Box holding the type.
    // This syntax takes ownership of Box<Self>, invalidating the old state so the state value of the Post can transform into a new state.
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    // We add a default implementation for the content method that returns an empty string slice.
    // That means we don’t need to implement content on the Draft and PendingReview structs.
    // The Published struct will override the content method and return the value in post.content.
    // Note that we need lifetime annotations on this method. We’re taking a reference to a post as an argument and returning a reference
    // to part of that post, so the lifetime of the returned reference is related to the lifetime of the post argument.
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    // The request_review method on Draft needs to return a new, boxed instance of a new PendingReview struct,
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    // Calling approve on a Draft has no effect
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    // The PendingReview struct also implements the request_review method but doesn’t do any transformations.
    // Rather, it returns itself, because when we request a review on a post already in the PendingReview state, it should stay in the PendingReview state.
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
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




// By implementing the state pattern exactly as it’s defined for object-oriented languages, we’re not taking as full advantage of Rust’s strengths as we could.
// Rather than encapsulating the states and transitions completely so outside code has no knowledge of them,
// we’ll encode the states into different types. Consequently, Rust’s type checking system will prevent
// attempts to use draft posts where only published posts are allowed by issuing a compiler error.

// Both the PostImproved and DraftPost structs have a private content field that stores the blog post text.
// The structs no longer have the state field because we’re moving the encoding of the state to the types of the structs.
// The PostImproved struct will represent a published post, and it has a content method that returns the content.

// Because the only way to get a published PostImproved instance that does have a content method defined is to call
// the approve method on a PendingReviewPost, and the only way to get a PendingReviewPost is to call the
// request_review method on a DraftPost, we’ve now encoded the blog post workflow into the type system.

// The transformations between the states are no longer encapsulated entirely within the Post implementation.
// However, our gain is that invalid states are now impossible because of the type system and the type checking that happens at compile time!

// Object-oriented patterns won’t always be the best solution in Rust due to certain features, like ownership, that object-oriented languages don’t have.
pub struct PostImproved {
    content: String,
}

pub struct DraftPost {
    content: String
}

impl PostImproved {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

// Note that DraftPost does not have a content method defined!
// So now the program ensures all posts start as draft posts, and draft posts don’t have their content available for display
// Any attempt to get around these constraints will result in a compiler error.
impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

// The PendingReviewPost struct doesn’t have a content method defined on it, so attempting to read its content results in a compiler error, as with DraftPost.
impl PendingReviewPost {
    pub fn approve(self) -> PostImproved {
        PostImproved {
            content: self.content
        }
    }
}