use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // constants are always immutable, the data type must be declared
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // shadowing + scopes
    let x = 5;

    let x = x + 1; // shadowing

    // inner shadowing
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); // 12
    }
    println!("The value of x is: {x}"); // 6

    // count number of spaces
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The number of spaces is: {spaces}");

    let guess: u32 = "42".parse().expect("Not a number!");

    // integer data types
    const DECIMAL_LITERAL: u32 = 98_222;
    const HEX_LITERAL: u32 = 0xff;
    const OCTAL_LITERAL: u32 = 0o77;
    const BINARY_LITERAL: u32 = 0b1111_0000;
    const BYTE_LITERAL: u8 = b'A';

    // floating-point types
    let x = 2.0; // f64 by default
    let y: f32 = 3.0;

    // numeric operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5; // Results in 3

    // boolean type
    let t = true;
    let f: bool = false;

    // character type
    let c = 'z';
    let z: char = 'Z'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // compound types - can group multiple values into one type
    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    // access tuple elements by index
    let fst = tup.0;
    let snd = tup.1;
    let thd = tup.2;

    // array type
    let a = [1, 2, 3, 4, 5]; // allocated on the stack

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // 3: initial value, 5: length
    let a = [3; 5];

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    // failed if the index is out of bounds
    let element = a[index];

    println!("The value of the element at index {index} is {element}");
}
