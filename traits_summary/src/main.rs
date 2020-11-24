use traits_summary::Summary;
use traits_summary::notify;

fn main() {
    let tweet = traits_summary::Tweet {
        username: String::from("horse_ebook"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    notify(&tweet);
}
