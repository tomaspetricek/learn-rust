struct User {
    active: bool, // each attribute is called a field
    username: String,
    email: String,
    sign_in_count: u64,
}

// tuple structs - the fields remain nameless
// are still strong types
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit-like fields
struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    println!("user1 active: {}", user1.active);

    let user2 = build_user(String::from("james@example.com"), String::from("james"));

    // struct update syntax
    // it moves the data, s user1 can no longer be used
    let user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let Point(x, y, z) = origin;

    // unit structs
    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        email: email,
        username: username,
        sign_in_count: 1,
    }
}
