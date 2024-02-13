pub mod english;
mod japanese;
pub mod largest;

pub trait Summary {
    fn summarise(&self) -> String ;
}

pub struct NewsArticle {
    pub content : String , 
    pub headline : String , 
    pub location : String , 
    pub author : String , 
}

impl Summary for NewsArticle {
    fn summarise(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarise(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}