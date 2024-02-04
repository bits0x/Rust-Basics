use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text(
     "Rust is a multi-paradigm, general-purpose programming language that emphasizes performance, type safety, and concurrency. It enforces memory safety"
    );

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!(
        "Rust is a multi-paradigm, general-purpose programming language that emphasizes performance, type safety, and concurrency. It enforces memory safety",
        post.content()
    );
}
