use std::cmp::Ordering;
// io - input/output library from std - standard library
use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // random generator that is local to the thread,
                                                               // is seeded by the operating system
                                                               // start..=end range expression

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // mut - makes it mutable, new - creates an empty string

        // io::stdin() - is a function that returns an instance of std::io::Stdin
        //             - type that represents handle to standard input
        io::stdin()
            .read_line(&mut guess) // append to the string - does not override, passed as mutable reference
            .expect("Failed to read line"); // returns a Result - which is an enum, variants are Ok and Err

        // shadows the previous declaration
        // trim - eliminates any white spaces at the beginning and end
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input entered");
                continue;
            }
        };
        println!("You guessed: {guess}"); // {} - is a placeholder

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
