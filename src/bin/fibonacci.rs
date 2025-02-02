use std::io;

fn main() {
    println!("Program that prints fibonacci numbers");
    loop {
        println!("Enter maximum number for sequence:");
        let mut max = String::new();
        io::stdin().read_line(&mut max).expect("Failed to read input");
        match max.trim().parse() {
            Ok(n) => {
                print_fibonacci(n);
                break
            },
            Err(_) => println!("Invalid number.") 
        }
    }
}

fn print_fibonacci(max: u128) {
    let mut prev = 0;
    let mut curr = 1;
    loop {
        print!("{curr} ");
        if prev + curr > max { break }
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
    println!();
}