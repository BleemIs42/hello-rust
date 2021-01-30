
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

fn main() {
    let username = String::from("Struct");
    let usr = User{
        username,
        email: String::from("email"),
        sign_in_count: 12,
        active: false
    };
    println!("usr => {:#?}", usr);
    println!("usr.username {}", usr.username);

    let usr2 = User{
        active: false,
        ..usr
    };
    println!("usr2 => {:#?}", usr2);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black => {:#?}", black);
    println!("origin => {:#?}", origin);
}
