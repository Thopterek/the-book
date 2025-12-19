use std::fs::{self, File};
// for error versions
use std::io::{self, Read, ErrorKind};
use std::error::Error;

// to use ? mark in main we can add trait object
// same C convention as per returning int values
// there is also option to work with Termination
// std::process::Termination trait
fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, Result");
    /*
     * enum Result<T, E> {
     *  Ok(T),
     *  Err(E),
     * }
     * T -> value returned with success
     * E -> the error variant
    */
    let greet_file_res = File::open("hello.txt");
    let _greet_file = match greet_file_res {
        Ok(file) => file,
        // Err(error) => panic!("Problem with openining {error:?}"),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem with creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        }
    };
    // different version with unwrap_or_else
    let _another_file = File::open("world.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("world.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
    // let wrap_calls_panick = File::open("bruuuh.txt").unwrap();
    // let expect_allows_for_message = File::open("wowowo.txt").expect("THIS IS MY MESSAGE");
    let _value = match you_can_return_err() {
        Err(e) => println!("We got the error {e:?}"),
        Ok(name) => println!("There is a name {name}"),
    };
    // all versions below work the same as above one
    let _val = match rewrite_with_question_mark() {
        Err(e) => println!("Error {e:?}"),
        Ok(name) => println!("Name {name}"),
    };
    let _chain = chained_read();
    let _short = shortest();
    let _greet = File::open("Hey.txt")?;
    Ok(())
}

// propagation errors moving the handling out of scope
fn you_can_return_err() -> Result<String, io::Error> {
    let user_file = File::open("hello.txt");
    let mut user = match user_file {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut name = String::new();
    match user.read_to_string(&mut name) {
        Ok(_) => Ok(name),
        Err(e) => Err(e),
    }
}

/*
 * Uses from function -> def as From trait in std
 * converting the error values to correct types
 * ? works also for Option<T> values
*/
fn rewrite_with_question_mark() -> Result<String, io::Error> {
    let mut user_file = File::open("hello.txt")?;
    let mut name = String::new();
    user_file.read_to_string(&mut name)?;
    Ok(name)
}
// there is option for chained version
fn chained_read() -> Result<String, io::Error> {
    let mut user = String::new();
    File::open("hello.txt")?.read_to_string(&mut user)?;
    Ok(user)
}

// or even more using the fs
fn shortest() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
