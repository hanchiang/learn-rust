// We’ll implement a blog post workflow in an incremental way. The blog’s final functionality will look like this:
// A blog post starts as an empty draft.
// When the draft is done, a review of the post is requested.
// When the post is approved, it gets published.
// Only published blog posts return content to print, so unapproved posts can’t accidentally be published.
// Any other changes attempted on a post should have no effect.
// For example, if we try to approve a draft blog post before we’ve requested a review, the post should remain an unpublished draft.

use object_oriented_design::{Post, PostImproved};

pub fn main() {
    post();
    post_improved();
}

fn post() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}

fn post_improved() {
    let mut post = PostImproved::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());

}