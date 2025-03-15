struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32); 
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    let email = user1.email;
    user1.email = String::from("newemail@example.com");

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}