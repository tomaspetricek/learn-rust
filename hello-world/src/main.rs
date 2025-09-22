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
//
// # Check Compilability
// cargo check
//
// # Compile without Optimizations
// build: cargo build --release
// run:   ./target/release/hello-world
fn main() {
    println!("Hello, world!"); // the ! is present because it is a macros not a function
}
