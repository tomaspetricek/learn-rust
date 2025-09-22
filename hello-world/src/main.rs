// # Run using compiler directly
// compile: rustc main.rs
// run:     ./main.rs
//
// # Run using Cargo
// ## Two Steps
// compile: cargo build
// run:     ./target/debug/hello-world
// ## Single Step
// cargo run
fn main() {
    println!("Hello, world!"); // the ! is present because it is a macros not a function
}
