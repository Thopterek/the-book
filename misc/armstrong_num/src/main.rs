use std::io;

fn armstrong_num() -> bool {
    return false
}

fn perfect_num() -> bool {
    return true
}

fn main() {
    println!("Choose the number to be checked:");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    println!("In the buffer we got -> {}", buffer.trim());
    let mut number: u32 = 0;
    println!("You have chosen -> {}", number);
    println!("The value of perfect_num is -> {}", perfect_num());
    println!("and of the armstrong_um is -> {}", armstrong_num());
    Ok(())
}
