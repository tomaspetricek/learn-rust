use aggregator::{notify, notify_v2, Book, SocialPost, Summary};

fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    };

    println!("1 new post: {}", post.summarize());

    let book = Book {
        author: String::from("Jay Shetty"),
        name: String::from("8 rules of love"),
        content: String::from("Love Letter to the World"),
    };

    println!("the book is about: {}", book.summarize());

    notify(&book);
    notify_v2(&post);

    let s = 10.to_string();
}
