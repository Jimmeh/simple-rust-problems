use std::io;

fn main() {
    println!("Program for calculating factorial values.");
    println!("Enter a number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input: u128 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => { 
            println!("Unable to parse number");
            0
        }
    };
    
    println!("Result: {}", factorial(input))
}

fn factorial(num: u128) -> u128 {
    if num <= 1 {1} else {num * factorial(num - 1)}
}
