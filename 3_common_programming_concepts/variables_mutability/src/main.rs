/*
 * opting out of constants
 * examples of mutability
*/
fn main() {
    println!("Setting up the new version");
    let mut x = 5;
    println!("The value of x -> {x}");
    /*
     * Allowed only through adding the mut keyword
    */
    x = 6;
    println!("The value of x -> {x}");
    /*
     * you can't use the mut for constants
     * beforehand are just variables
     * they can be declared in any scope
     * UPPERCASE_WITH_UNDERSCORE_BETWEEN
    */
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of 3h in seconds -> {THREE_HOURS_IN_SECONDS}");
    /*
     * shadowing is possible, its creating new instances
     * thats why in this way we are not modifying the original
    */
    let shadow = 5;
    println!("Shadow at the beginning is {shadow}");
    let shadow = shadow + shadow;
    println!("Shadowed version adding them together {shadow}");
    {
        /*
         * lifetime of the variable is only in the scope of brackets
        */
        let shadow = shadow + 6;
        println!("Inner shadow value is {shadow}");
    }
    println!("But the the outer variable has value {shadow}");
    /*
     * this allows for conversions through shadowing
     * rather than making something mutable
    */
    let spaces = "   ";
    println!("Spaces value between dashes -{spaces}-");
    let spaces = spaces.len();
    println!("Spaces value between dashes -{spaces}-");
}
