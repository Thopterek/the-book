use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    println!("Hello, scope work!");
    let plant = Asparagus {};
    println!("I'm growing {plant:?}");
}
