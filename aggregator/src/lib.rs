use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)") // provide default behiviour
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

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub struct Book {
    pub author: String,
    pub name: String,
    pub content: String,
}

impl Summary for Book {}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_v2<T: Summary>(item: &T) {
    println!("Breaking news!: {}", item.summarize());
}

// enable different types to be passed
pub fn notify_v3(item1: &impl Summary, item2: &impl Summary) {}

// enforce passing of same types
pub fn notify_v4<T: Summary>(item1: &T, item2: &T) {}

// specify multiple traits
pub fn notify_v5(item: &(impl Summary + Display)) {}

pub fn notify_v6<T: Summary + Display>(item: &T) {}

pub fn some_function_v1<T: Display + Clone, U: Display + Debug>(t: &T, u: &U) -> i32 {
    42
}

pub fn some_function_v2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Display + Debug,
{
    42
}

fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    }
}

pub struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest meber is y = {}", self.y);
        }
    }
}

pub trait F32Extras {
    fn squared(self) -> f32;
    fn is_near_zero(self, eps: f32) -> bool;
}

impl F32Extras for f32 {
    fn squared(self) -> f32 {
        self * self
    }

    fn is_near_zero(self, eps: f32) -> bool {
        self.abs() < eps
    }
}
