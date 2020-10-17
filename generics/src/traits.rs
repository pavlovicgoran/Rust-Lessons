use std::fmt::Display;
// How to define a trait
pub trait Summary {
    // default implementation
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub retweet: bool,
    pub reply: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Same as `pub fn notify<T: Summary>(summary: &T) -> String`
pub fn notify(summary: &dyn Summary) -> String {
    format!("Breaking news!!! {}", summary.summarize())
}

// Combining traits with +
pub fn notify_and_display<T: Summary + Display>(item: &T) -> String {
    format!("Display => {}", item.summarize())
}

// Example to return a trait from a function
// the function needs to return only one type instances
// it can't return Tweets or NewsArticles
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
