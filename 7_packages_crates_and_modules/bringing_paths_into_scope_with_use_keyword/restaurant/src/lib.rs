/*
 * The auto generated part
*/
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

/*
 * Following the code example
 * out of it we make module tree
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
 * showcasing how they are nested
 * serving / hosting would be sibling modules
*/
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

// simplify the path needed to be taken
// working only in the scope its present in
// it won't apply to another mod even defined here
use crate::front_of_house::hosting;

// different when bringing two items with same name
use std::fmt;
use std::io;
// then calling fmt::Result or io::Result<()> making it explicit
// rather than bringing in use::std::fmt::Result and the other one
// you can also bring them as other names to handle it
use std::fmt::Result;
// adding pub in front to allow for re exporting
pub use std::io::Result as IoResult;
// bringing things from the same crate / module you can use nested path
// rather than use std::cmp::Ordering and use::std::io
use std::{cmp::Ordering, io};
// or using something like self for two verison of use in one statement
use std::io::{self, Write};
// if bringing everything we should use glob operator *
// mostly just used for testing module
use std::collections::*;

// using super as a way to reference something in parent module
fn deliver_order() {}

mod back_of_house {
    pub enum Appetize {
        Soup,
        Salad,
    }
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    // function to set value because seasonal_fruit is not pub
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    fn cook_order() {}
}

// using structs (follow strict rules) and enums (only it needs to be pub, default) with pub
pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    let order = back_of_house::Appetize::Soup;
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // relative path, replaced the below one thanks to use
    // front_of_house::hosting::add_to_waitlist();
    // keeping hosting to see where its defined
    hosting::add_to_waitlist();
}

