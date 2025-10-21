use learn_rust_art::{PrimaryColor, mix};

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    let res = mix(&red, &yellow);
    println!("The result of mixing {red:#?} and {yellow:#?} is: {res:#?}");
}