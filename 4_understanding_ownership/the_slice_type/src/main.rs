fn main() {
    println!("What the hell is a slice type?");
    let mut skrr = String::from("Hello Everyone");
    let word = first_word(&skrr);
    println!("First word returned -> {word}, and sentence is {skrr}");
    skrr.clear(); // empty the string
    println!("After cleaning the word is still {word}, but sentence is '{skrr}'");
    {
        /*
         * Slices as a way to keep those things in sync
         * as per taking the reference only to part of string
         * &[variable][staring_index..ending_index];
        */
        let text = String::from("Hello World");
        let hello = &text[0..5];
        let world = &text[6..11];
        println!("Using of slices from {text} -> {world} -> {hello}");
        let same = &text[..5];
        let thing = &text[5..];
        println!("And using diff way to write in syntax -> {thing} -> {same}");
    }
    let mut eyo = String::from("Another one");
    let got_a_slice = return_slice_word(&eyo);
    println!("{eyo}, with a first word -> {got_a_slice}");
    eyo.clear();
    println!("So now {eyo} and we can't use got_a_slice as it is invalidated");
    {
        // specific point of the binary
        let _s: &str = "Hello World";
        // as per signature better to use &str if avaiable
        let a = [1, 2, 3];
        let _slice_of_a = &a[1..2];
    }

}

/*
 * actual way to return the word
 * prefering &str rather than String
*/
fn return_slice_word(string: &str) -> &str {
    let bytes = string.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &string[0..i];
        }
    }
    &string[..]
}

/*
 * Takes a string of words with seperator
 * returns the first word it finds
 * or the whole string
*/
fn first_word(string: &String) -> usize {
    // conversion to array of bytes
    let bytes = string.as_bytes();
    // iter creates iterator over array of bytes
    // enumarate wraps it and returns in tuple
    // 1st element -> index, 2nd element -> reference to value
    // using it to destructure the tuple through (i, &item)
    for (i, &item) in bytes.iter().enumerate() {
        // b signifies the bytes syntax
        if item ==b' ' {
            return i;
        }
    }
    string.len()
}
