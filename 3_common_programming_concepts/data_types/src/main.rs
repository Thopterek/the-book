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
    */
    let eight: i8 = 127;
    let sixtee: i16 = 32767;
    println!("8 bit max is {eight} and then follows 16 bit -> {sixtee} till architecture max");
    let un_eight: u8 = 255;
    let un_sixtee: u16 = 65535;
    println!("for unsigned goes from 8 bit -> {un_eight}, then 16 bit -> {un_sixtee} and so on");
}
