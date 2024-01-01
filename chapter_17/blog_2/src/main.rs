use blog_2::Post;

/// The changes we needed to make to main to reassign post mean that this implementation doesn’t
/// quite follow the object-oriented state pattern anymore: the transformations between the states
/// are no longer encapsulated entirely within the Post implementation. However, our gain is that
/// invalid states are now impossible because of the type system and the type checking that happens
/// at compile time! This ensures that certain bugs, such as display of the content of an
/// unpublished post, will be discovered before they make it to production.
fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}
