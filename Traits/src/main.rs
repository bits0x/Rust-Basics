
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String
}

impl Summary for NewsArticle {}

impl Summary for Tweet {
    fn summarize(&self) -> String { // override impl
        format!("{}: {}", self.username, self.content)
    }
}

// we can use the trait to define the shared behaviour b/w
// Newsarticle and tweet for the summarization fn
pub trait Summary {
    fn summarize(&self) -> String { // default impl
        format!("Read More ... ") 
    }
}

fn returns_summarizable() -> impl Summary { // returns any type that implements summary trait
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false
    }
}

// Trait Bounds , use trait as param
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking News! {}", item.summarize());
}

fn main() {
    let tweet = Tweet {
        username: String::from("@john"),
        content: String::from("Hello World"),
        reply: false,
        retweet: false
    };

    let artice = NewsArticle {
        author: String::from("john"),
        headline: String::from("The sky is falling"),
        content: String::from("The sky is not exactly falling")
    };

    println!("Tweet summary: {}", tweet.summarize());   // @john: Hello world
    println!("Tweet summary: {}", artice.summarize());  // Read More ...
    notify(&artice); // Breaking News! Read More ...

    println!("{}", returns_summarizable().summarize()); // horse ebooks: of course ... 
}