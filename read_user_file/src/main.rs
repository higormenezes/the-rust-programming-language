use std::{fs, io};

fn main() {
    let username = read_username_from_file().expect("Fail to read username");
    println!("username {username}");
}

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
