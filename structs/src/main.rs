#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sing_in_count: u64,
}

fn main() {
    let user = User {
        active: true,
        username: String::from("someone"),
        email: String::from("someone@example.com"),
        sing_in_count: 1,
    };

    println!("{:#?}", user);
}
