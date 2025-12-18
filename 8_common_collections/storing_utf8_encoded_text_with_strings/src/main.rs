fn main() {
    println!("Hello, Strings!");
    let mut mutable_string = String::new();
    let data = "initial literal str".to_string();
    let s = String::from("initial literal str");
    if data == s {
        println!("Same things");
    }
    /*
     * Updating the String
     * - push_str(slice)
     * - push(char)
    */
    mutable_string.push_str("Hey ");
    mutable_string.push('U');
    println!("{}", mutable_string);
}
