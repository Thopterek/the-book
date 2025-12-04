/*
 * C style if statements
 * but they have to have {}
 * Conditions have to be bools
*/
fn main() {
    let num: i8 = 7;
    if num < 5 {
        println!("Condition is true");
    }
    else if num == 5 {
        println!("It was the extra one");
    }
    else {
        println!("Condition is false");
    }
    /*
     * there is also if in the let statements
     * if [Condition] {value_if_condition} else {value_if_not}
     * it has to the same type in both of the values
    */
    let bigger = if num > 5 {10} else {0};
    println!("Bigger is equal to -> {bigger}");
    /*
     * there are 3 types of loops:
     * loop / while / true
     * first one is infinite till break
     * you can return values from it in let statement
    */
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Returned result from loop is -> {result}");
    /*
     * You can add labels to the loop starting with '
     * just to be more explicit about them to diff it
    */
    let mut cc = 0;
    'counting_two: loop {
        cc += 1;
        'print_each_time: loop {
            println!("Hello we are printing in inner loop");
            break 'print_each_time;
        }
        if cc == 2 {
            break 'counting_two;
        }
    }
    /*
     * Basic while loop in C style
    */
    let mut small_cc: i8 = 0;
    while small_cc != 2 {
        println!("Number {small_cc}");
        small_cc += 1;
    }
    let collection = [10, 20, 30];
    let mut index = 0;
    while index < 3 {
        println!("The value of {index} is {}", collection[index]);
        index += 1;
    }
    /*
     * and Python style for loop with and without ranges
     * with possible addition of reverse method
    */
    for element in collection {
        println!("The element is {element}");
    }
    for reverse in (1..3).rev() {
        println!("Reverse 1 to 3 is {reverse}");
    }
}
