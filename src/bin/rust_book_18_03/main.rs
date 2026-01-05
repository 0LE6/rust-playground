// Implementing an 
// Object-Oriented Design Pattern
mod lib;
use lib::Post;

fn main() {
    let mut post = Post::new();
    
    post.add_text("I ate a pizza for lunch today");
    assert_eq!("", post.constent());

    post.request_review();
    assert_eq!("", post.constent());

    post.approve();
    assert_eq!(
        "I ate a pizza for lunch today",
        post.constent()
    );
}
