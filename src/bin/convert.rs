use std::io;


fn main() {
    println!("Program for converting GBP to USD.");
    let conversion = prompt_for_conversion();
    let amount = prompt_for_amount();
    println!("{}{:.2} is {}{:.2}", conversion.from, amount, conversion.to, amount * conversion.multiplier)
}

fn prompt_for_amount() -> f32 {
    println!("How much do you want to convert?");
    loop {
        let mut amount = String::new();
        io::stdin().read_line(&mut amount).expect("Failed to read input");
        match amount.trim().parse() {
            Ok(n) => return n,
            Err(_) => {
                println!("Must be a valid decimal number. Try again:");
                continue
            }
        };
    }
}

const GBP_PER_USD: f32 = 0.9;
struct Conversion {
    from: char,
    to: char,
    multiplier: f32
}

fn prompt_for_conversion() -> Conversion {
    println!("Which currency do you have?");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        match input.trim() {
            "USD" => return Conversion { from: '$', to: '£', multiplier: GBP_PER_USD },
            "GBP" => return Conversion { from: '£', to: '$', multiplier: 1f32/GBP_PER_USD },
            _ => {
                println!("Valid currencies are [USD, GBP]. Try again:");
                continue
            }
        }
    }
}