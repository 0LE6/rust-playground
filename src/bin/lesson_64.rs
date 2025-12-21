// 64: Traits in Rust 

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub replay: bool,
    pub repost: bool,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!(
            "{}, by {} ({})",
            self.headline,
            self.author,
            self.location
        )
    }
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!(
            "{} : {}",
            self.username,
            self.content,
        )
    }
}

fn main() {
    let article = NewsArticle {
        headline: String::from(
            "Cacao for cocoa!"
        ),
        location: String::from("Cocoa Island"),
        author: String::from("Coco Channel"),
        content: String::from(
            "Marvelous coco experience!"
        )
    }; 

    let post = SocialPost {
        username: String::from(
            "Cacao magazine"
        ),
        content: String::from("Cocoa Island"),
        replay: false,
        repost: false,
    };

    println!("{}", article.summarize());
    println!("{}", post.summarize());
}
