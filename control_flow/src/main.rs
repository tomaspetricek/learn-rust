fn main() {
    // if expressions
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("confition was false");
    }
    println!("confition was {}", number < 5);

    // next: Handling Multiple Conditions with else if
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3"); // should be printed out
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3 or 2");
    }

    // using if as an expression
    let condition = true;
    let number = if condition { 5 } else { 3 };
    println!("The value of number is: {number}");

    // loops
    loop {
        print!("again!");
        break;
    }

    // returning values from the loop
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is: {result}");

    // loop labels
    let mut count = 0;
    'counting_loop: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining: {remaining}");

            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_loop;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("enc count = {count}");

    // while loops
    let mut number = 3;

    while number != 0 {
        println!("number = {number}");
        number -= 1;
    }
    println!("end of while loop");

    let a = [1, 2, 3, 4, 5];

    let mut index = 0;

    while index < 5 {
        println!("the value at index = {index} is {}", a[index]);
        index += 1;
    }

    // for each
    for element in a {
        println!("the value is: {element}");
    }
    // use range
    for number in (1..4).rev() {
        println!("number = {number}");
    }
}
