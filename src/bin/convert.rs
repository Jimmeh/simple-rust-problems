use std::io;

const GBP_PER_USD: f32 = 0.9;

fn main() {
    println!("Program for converting GBP to USD.");
    let from_curr = read_from_curr();
    let amount = read_amount();
    if from_curr == "GBP" {
        println!("{} USD", amount / GBP_PER_USD)
    } else {
        println!("{} GBP", amount * GBP_PER_USD)
    }
}

fn read_amount() -> f32 {
    println!("How much do you want to convert?");
    let mut amount = String::new();
    loop {
        io::stdin()
            .read_line(&mut amount)
            .expect("Failed to read input");
        
        let amount: f32 = match amount.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Must be a valid decimal number. Try again:");
                continue
            }
        };
        return amount;
    }
}

fn read_from_curr() -> String {
    println!("Which currency do you have?");
    let mut input = String::new();
    loop {
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let from_curr = input.trim();
        if from_curr == "USD" || from_curr == "GBP" {
            return from_curr.to_string()
        }
        println!("Valid currencies are [USD, GBP]. Try again:");
    }
}