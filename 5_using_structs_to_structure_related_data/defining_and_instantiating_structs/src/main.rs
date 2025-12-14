/*
 * as of now looks like C style struct
 * but then uses key -> value pairs
 * to fill out or access values
*/
struct User {
    active: bool,
    username: String,
    email: String,
    _sign_in_count: u64,
}

fn create_user(active: bool, username: String, email: String, count: u64) -> User {
    let new_user = User {
        active: active,
        username: username,
        email: email,
        _sign_in_count: count,
    };
    new_user
}

/*
 * We can rewrite above function with Field init
 * if it takes the exact same name it fills out on its own
*/
fn field_init_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        _sign_in_count: 1,
    }
}

fn main() {
    println!("Hello, structs!");
    let user_one = create_user(true, String::from("first_dude"), String::from("some@email"), 2);
    println!("Status of user one -> {} is active -> {}",user_one.username, user_one.active);
    let mut user_two = create_user(false, String::from("ANOTHER DUDE"), String::from("no_email"), 0);
    println!("And now of the second user with email -> {}, active -> {}", user_two.email, user_two.active);
    user_two.active = true;
    println!("And then got changed to active -> {}, but email is still -> {}", user_two.active, user_two.email);
    let filled_user = field_init_user(String::from("FILL UP"), String::from("Random Ass Email"));
    println!("Damn it works -> {}", filled_user.username);
    // Struct update syntax, allowing to fill out only a part to be changed
    // but it still moves things around that don't have the copy trait like String
    let remade_two = User {
        email: String::from("WELL HE HAS EMAIL"),
        username: String::from("Added Name"),
        ..user_two
    };
    println!("Lets see remate_two -> {}, {}, {}", remade_two.email, remade_two.active, remade_two.username);
    println!("Lets see user_two -> {}, {}, {}", user_two.email, user_two.active, user_two.username);
    let pointxyz = Numbers(0, 0, 0);
    // deconstruct but naming the type
    let Numbers(x, y, z) = pointxyz;
    println!("{x}, {y}, {z}");
    let same = SameButDiff(0, 0, 0);
    let SameButDiff(a, b, c) = same;
    println!("{a}, {b}, {c}");
    let _idk = WhatsGoingOn;
}

/*
 * Those are Tuple Structs
 * each has different type depending on struct name
*/
struct Numbers(i32, i32, i32);
struct SameButDiff(i32, i32, i32);

/*
 * Unit Like Structs
 * this thing type -> ()
 * implementing a trait on some type but no storage
*/
struct WhatsGoingOn;

