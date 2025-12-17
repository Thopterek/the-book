use std::any::type_name;

// extra type checking
fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    println!("--------------");
    println!("Hello, if let!");
    println!("--------------");
    /*
     * Handling one particular pattern
     * if not ignoring the rest
     * if let [pattern(value)] = [to_be_checked] {
     * binding to whats there to value
     * [code]
     * }
    */
    let config_max = Some(3u8);
    println!("{}", type_of(&config_max));
    if let Some(max) = config_max {
        println!("The max is configured to be {max}");
    }
    if let None = config_max {
        println!("It was none");
    } else {
        println!("Its something");
    }
    // match version to compare to
    match config_max {
        Some(max) => println!("The max is configured to be {max}"),
        _ => (),
    }
    // there is also let else as per
    if let None = check(config_max) {
        println!("There was nothing");
    } else {
        println!("There was a string returned");
    }
}

fn check(passed: Option<u8>) -> Option<String> {
    let Some(value) = passed else {
        return None;
    };
    if value > 1 {
        Some("Oh its bigger than one its".to_string())
    } else {
        Some("It was smaller than 1".to_string())
    }
}
