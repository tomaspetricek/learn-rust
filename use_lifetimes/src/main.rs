use std::fmt::Display;

// the instance of ImportantExcerpt cannot outlive the reference it holds in its part field
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

fn main() {
    println!("Hello, world!");

    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is: {result}");

    let novel = String::from("Call me Ishamel, Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    let first = first_word(novel.as_str());
    assert_eq!(first, "Call");
    let level = i.level();
    i.announce_and_return_part("hello");

    // static lifetime
    let s: &'static str = "I have a static lifetime.";
    longest_with_an_annuncement(string1.as_str(), string2, "hello");
}

// the return reference will be valid as long as both the parameters are valid
// the string slice returned from the function will live at least as long as lifetime 'a
// the lifetime of the reference returned by the function is the same as the smaller of the time lifetimes of the references passed in
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// the compiler can infer the reference lifitimes
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn longest_with_an_annuncement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
