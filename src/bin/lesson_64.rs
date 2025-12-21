// 64: Traits in Rust 

use std::{io::Stdin, path::StripPrefixError};

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

// pub struct NewsArticle {
//     pub headline: String,
//     pub location: String,
//     pub author: String,
//     pub content: String,
// }
//
// pub struct SocialPost {
//     pub username: String,
//     pub content: String,
//     pub replay: bool,
//     pub repost: bool,
// }
//
// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!(
//             "{}, by {} ({})",
//             self.headline,
//             self.author,
//             self.location
//         )
//     }
// }
//
// impl Summary for SocialPost {
//     fn summarize(&self) -> String {
//         format!(
//             "{} : {}",
//             self.username,
//             self.content,
//         )
//     }
// }

struct SimpleNote {
    content: String,
}

struct DetailedPost {
    username: String, 
    content: String,
}

impl Summary for SimpleNote {}

impl Summary for DetailedPost {
    fn summarize(&self) -> String {
        format!(
            "{} : {}",
            self.username,
            self.content
        )
    }
}
fn main() {
    let note = SimpleNote {
        content: String::from(
            "This is a simple note"
        ),
    };

    let post = DetailedPost {
        username: String::from("user123"),
        content: String::from("Hello, world!"),
    };

    println!("Note {}", note.summarize());
    println!("Note {}", post.summarize());

    // let article = NewsArticle {
    //     headline: String::from(
    //         "Cacao for cocoa!"
    //     ),
    //     location: String::from("Cocoa Island"),
    //     author: String::from("Coco Channel"),
    //     content: String::from(
    //         "Marvelous coco experience!"
    //     )
    // }; 
    //
    // let post = SocialPost {
    //     username: String::from(
    //         "Cacao magazine"
    //     ),
    //     content: String::from("Cocoa Island"),
    //     replay: false,
    //     repost: false,
    // };
    //
    // println!("{}", article.summarize());
    // println!("{}", post.summarize());
}
