/*
 * without use (includes), we load prelude set
*/
use std::io;
/*
 * use is our header / import
 * std -> standard library
 * io <- input / output
*/

fn main() {
    println!("Guest the number!\nPlease input your guess.");
    /*
     * to make variable mutable you have to add mut
     * if not -> everything is being constant
     * [new_instance]::[associated_function]
     * in this instance creating empty str
    */
    let mut guess = String::new();
    /*
     * possible rewrite (without use) std::io::stdin
     * create a new instance of std::io::Stdin
    */
    io::stdin()
        /*
         * appends the line into a string
         * uses references -> same  logic:
         *  - they have to be made mutable
         *  - if not things are constants
        */
        .read_line(&mut guess)
        /*
         * uses C like enum for errors
         * return (0) == Result Ok
         * return (23) == Result Err
        */
        .expect("Error: Failed to read line.");
    println!("You guessed: {guess}");
}
