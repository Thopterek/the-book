/*
 * as of now looks like C style struct
 * but then uses key -> value pairs
 * to fill out or access values
*/
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn create_user(active: bool, username: String, email: String, count: u64) -> User {
    let new_user = User {
        active: active,
        username: username,
        email: email,
        sign_in_count: count,
    };
    new_user
}

fn main() {
    println!("Hello, structs!");
    let user_one = create_user(true, String::from("first_dude"), String::from("some@email"), 2);
    println!("Status of user one -> {} is active -> {}",user_one.username, user_one.active);
    let mut user_two = create_user(false, String::from("ANOTHER DUDE"), String::from("no_email"), 0);
    println!("And now of the second user with email -> {}, active -> {}", user_two.email, user_two.active);
    user_two.active = true;
    println!("And then got changed to active -> {}, but email is still -> {}", user_two.active, user_two.email);
}
