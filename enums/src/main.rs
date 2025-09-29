enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr1 {
    kind: IpAddrKind,
    address: String,
}

enum IpAddr2 {
    V4(String),
    V6(String),
}

enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

enum Option<T> {
    None,
    Some(T),
}

enum Coin1 {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: &Coin1) -> u8 {
    match coin {
        Coin1::Penny => 1,
        Coin1::Nickel => 5,
        Coin1::Dime => 10,
        Coin1::Quarter => 25,
    }
}

impl Coin1 {
    fn value_in_cents(&self) -> u8 {
        value_in_cents(self)
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin2 {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin2::Penny => 1,
            Coin2::Nickel => 5,
            Coin2::Dime => 10,
            Coin2::Quarter(state) => {
                println!("state quarter from {state:?}");
                25
            }
        }
    }
}

// ToDo: find out why does it not compile
// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => i + 1
//     }
// }

fn main() {
    println!("Hello, world!");

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);

    let home = IpAddr1 {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr1 {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback = IpAddr2::V6(String::from("::1"));

    let home = IpAddr3::V4(127, 0, 0, 1);
    let loopback = IpAddr3::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');

    println!(
        "value of nickel in cents is: {}",
        Coin1::Nickel.value_in_cents()
    );
    println!(
        "value of quarter in cents is: {}",
        Coin2::Quarter(UsState::Alaska).value_in_cents()
    );
    // println!("plus one: {}", plus_one(Some(1)));

    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
    let config_max = Some(3u8);

    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}

fn route(ip: IpAddrKind) {}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}

fn describe_state_quarter1(coin: Coin2) -> Option<String> {
    if let Coin2::Quarter(state) = coin {
        if state.existed_in(1900) {
            Option::<String>::Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Option::<String>::Some(format!("{state:?} is relatively new."))
        }
    } else {
        Option::<String>::None
    }
}

fn describe_state_quarter2(coin: Coin2) -> Option<String> {
    // if let
    let state = if let Coin2::Quarter(state) = coin {
        state
    } else {
        return Option::<String>::None;
    };

    if state.existed_in(1900) {
        Option::<String>::Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Option::<String>::Some(format!("{state:?} is relatively new."))
    }
}

fn describe_state_quarter3(coin: Coin2) -> Option<String> {
    // let...else
    let Coin2::Quarter(state) = coin else {
        return Option::<String>::None;
    };

    if state.existed_in(1900) {
        Option::<String>::Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Option::<String>::Some(format!("{state:?} is relatively new."))
    }
}
