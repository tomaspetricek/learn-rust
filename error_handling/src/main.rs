// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

use std::fs::{self, File};
use std::io::ErrorKind;
use std::io::{self, Read};
use std::net::IpAddr;


pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("guess value must be in range 1 to 100, got {value}");
        }
        Guess{value}
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("Hello, world!");
    // unrecoverable errors
    // panic!("crash and burn");

    let v = vec![1, 2, 3];

    // v[99];

    // recoverable errors
    let file_opened = File::open("hello.txt");

    // let file = match file_opened {
    //     Ok(file) => file,
    //     Err(error) => panic!("failed to open a file due to: {error:?}"),
    // };

    let file = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(error) => panic!("problem creating the file: {error:?}"),
            },
            _ => {
                panic!("problem opening the file: {error:?}");
            }
        },
    };

    // closures
    let file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("problem creating the file: {error:?}");
            })
        } else {
            panic!("problem opening the file: {error:?}");
        }
    });

    let file = File::open("not_exist.txt").unwrap();

    let file = File::open("hello.txt").expect("hello.txt should be included in this project");

    // when code panics there is no way to recover
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("hardcoded ip address should be valid"); // shall never fail

    let guess = Guess::new(100);
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_opened = File::open("username.txt");

    let mut username_file = match username_file_opened {
        Ok(file) => file,
        Err(error) => return Err(error),
    };
    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(error) => Err(error),
    }
}

// ? operator for propagating errors
fn read_username_from_file_v2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_v3() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_v4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
