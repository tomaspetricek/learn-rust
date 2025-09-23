fn main() {
    println!("Hello, world!");
    another_function();
    take_parameter(3);
    print_labeled_measurement(5, 'h');

    // expression
    let y = {
        let x = 3;
        x + 1 // epxressions do NOT include a semicolon at the end
    };
    println!("The value of y is: {y}");

    let x = five();
    let x = plus_one(five());
    println!("The value of x is: {x}");
}

fn another_function() {
    println!("another function");
}

fn take_parameter(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
