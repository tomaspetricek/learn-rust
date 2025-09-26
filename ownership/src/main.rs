use std::{net::ToSocketAddrs, str::FromStr};

fn main() {
    {
        // s is not valid here, since it's not yet declared
        let s = "hello"; // s is valid from this point forward

        // do stuff with s
    } // this scope is now over, and s is no longer valid

    // create dynamically allocated string from a literal
    // uses RAII
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{s}");

    let s1 = String::from("hello");
    let s2 = s1; // invalidates s1, content of s1 is moved to s2
    println!("{s2}");

    let mut s = String::from("hello");
    s = String::from("ahoy"); // previously assigned string gets destroyed
    println!("{s}, world!");

    // cloning - making a deep copy
    let s1 = String::from("hello");
    let s2 = s1.clone(); // making a deep copy
    println!("s1 = {s1}, s2 = {s2}");

    let x = 7;
    let y = x.clone(); // possible, but not needed, since the size is known at compile-time and it is entirely stored on stack

    // ownership and functions
    let s = String::from("hello");
    takes_ownerhip(s); // s's value moves into the function -> so it is no longer valid here

    let x = 5;
    makes_copy(x); // because i32 implements the Copy trait, x does NOT move into the function, so it's okay to use it afterwards

    // return values and ownership
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

    // compute length
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");

    // references and borrowing
    let length = calculate_length_from_reference(&s2);

    // action of creating a reference is called borrowing

    // mutable references
    let mut s = String::from("hello");
    change(&mut s);

    // rust guarantees that references will never be dangling references
    // rules
    // - at any given time, you can have either one mutable reference or any number immutable references
    // - reference must always be valid

    // slices
    // - let you reference a contiguos sequence of elements in a collection
    // - is kind of reference, so it does not have a ownership
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    let first = first_word(&s[..]);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn takes_ownerhip(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // some string_string goes out of scope and 'drop' is called

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // some_integer goes out of scope

fn gives_ownership() -> String {
    String::from("yours")
}

fn takes_and_gives_back(source: String) -> String {
    source
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_from_reference(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word_last_index(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return index;
        }
    }
    s.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..index];
        }
    }
    &s[..]
}
