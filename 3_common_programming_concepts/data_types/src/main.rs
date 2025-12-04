/*
 * More in depth about type of language
 * as Rust is statically typed (compile)
*/
fn main() {
    println!("Data Types!");
    /*
     * without type annotation we get error
    */
    let guess: u32 = "42".parse().expect("Not a number");
    println!("guess - > {guess}");
    /*
     * using scalar types which are:
     * ints, floats, bools, chars
    */
    let small: i8 = 2;
    let floo: f32 = 3.3;
    let bully: bool = true;
    let charizard: char = 'a';
    println!("like int -> {small}, float -> {floo}, bool -> {bully}, char -> {charizard}");
    /*
     * Different types of ints are based on the possible size
     * setting from unsigned to standard by i or u prefix
     * 8, 16, 32, 64, 128 bit and till isize / usize
     * default goes to the i32
    */
    let eight: i8 = 127;
    let sixtee: i16 = 32767;
    println!("8 bit max is {eight} and then follows 16 bit -> {sixtee} till architecture max");
    let un_eight: u8 = 255;
    let un_sixtee: u16 = 65535;
    println!("for unsigned goes from 8 bit -> {un_eight}, then 16 bit -> {un_sixtee} and so on");
    /*
     * You can write numbers with _ as per 8_0_8
     * to have a visual seperation and takes in formts:
     * Decimal, Hex, Octal, Binary, Byte
    */
    let dec: i32 = 8_0_8;
    let hex: i32 = 0xff;
    println!("Dec is {dec}, Hex is {hex}, printed as decimals");
    /*
     * There ways to check for the overflows
     * as per handling of it rather than expecing wrap
     * checked fails with the explicit type
     * overflowing returns value and boolean
    */
    let new: i8 = eight.wrapping_add(1);
    println!("Overflow but wrapped_add -> {new}");
    {
        let test = eight.checked_add(1);
        println!("Now with checked_add is {:?}", test);
    }
    let (value, state) = eight.overflowing_add(1);
    println!("Value -> {} & State -> {}", value, state);
    {
        let test = eight.saturating_add(1);
        println!("Saturating add -> {test}");
    }
    /*
     * Floating point numbers are always signed
     * base one is f64 but there is also f32
     * same C++ standard IEEE754
     * division truncates towards zero
    */
    let x = 2.0;
    let y: f32 = 3.1;
    println!("Different floats addition {}", x + y);
    /*
     * Explicit type annotation can be done also for bools
     * and the chars, the second one are c style ''
     * if the values are unused you can prefix with _
     * using the UCI standard rather than ASCII
    */
    let prawda = true;
    let _falsz: bool = false;
    let _character = 'c';
    let explicit_char: char = 'c';
    println!("Chars are like in {explicit_char} -> {prawda}");
    /*
     * Compund types, grouping multiple values in one type
     * as for the basics we got tuple / array
     * tuples have fixed lenght made on declaration
     * you can explicitly type annotate through ([type], [another])
     * the first part is called binding -> creating compund element
     * to get individual we need to do destructure through another instance
     * another way is to use period and the index of the accessed value
     * tuple that is empty is an unit -> written as () -> empty value or empty return type
    */
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (first, second, third) = tup;
    println!("Can you print tuple? -> {first}, {second}, {third}\nYess by accessing individualy");
    println!("Or with the small acces {}", tup.1);
    /*
     * Arrays in rust also have fixed lenght
     * they have to have the same type like in C
     * lives on the stack unlike Vector
     * access through the [] in C style
     * to make it explicit let [name]: [[type]; [size]] = [values, more_val]
     * or with the same value through let [name]: [[value]; [how_many]]
     * rust panics (exits) before accessing the memory over the bounds
    */
    let array = [1, 2, 3];
    let array: [i32; 3] = array;
    let array = [3; 3];
    println!("Simple array with last value -> {}", array[2]);
}

