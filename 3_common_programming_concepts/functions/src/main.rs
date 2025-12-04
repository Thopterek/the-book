/*
 * parameters of the functions are given names
 * the concrete values are the arguments
*/
fn function_with_params(number: i32, label: char) {
    println!("The passed value is {number} & char {label}");
}

fn main() {
    small_function();
    function_with_params(25, 'a');
    let number: i32 = implicit_return();
    println!("The value from fn -> {number}");
    println!("and with explicit return {}", with_return());
}

/*
 * Through convention usage of snake_case
 * can be defined anywhere in the same scope
*/
fn small_function() {
    println!("Writing from function!");
}
/*
 * Statements are actions that do not return a value
 * eg. let number: i32 = 5;
 * Expressions evaluate to a resultant value
 * eg. calling functions, macros, new curly block
*/
fn implicit_return() -> i32 {
    5
}

/*
 * return keyword can provide earlier return
 * for fn you need to specify always the type of param
 * without ; at the end it notes the value to be returned
 * if ; is added if only value is there it makes it a statement
*/
fn with_return() -> i32 {
    return 10;
}
