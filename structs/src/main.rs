struct User {
    active: bool,
    username: String,
    email: String,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("vgnshiyer"),
        email: String::from("viyer@email.com"),
    };

    // let email = user1.email; // email is moved (var email owns it now)

    let user2 = User {
        ..user1
    };

    print!("hey");
}
