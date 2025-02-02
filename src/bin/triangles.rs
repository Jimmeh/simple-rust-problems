use std::io;

fn main() {
    println!("Program that draws triangles to the console.");
    let length = prompt_for_length();
    draw(length);
}

fn draw(length: usize) {
    println!("{}", "*".repeat(length));
    if length > 1 {
        draw(length - 1);
    }
}

fn prompt_for_length() -> usize {
    println!("Enter the length of the longest side:");
    let mut length = String::new(); 
    io::stdin().read_line(&mut length).expect("Failed to read input");
    loop {
        match length.trim().parse() {
            Ok(n) => return n,
            Err(_) => {
                println!("Failed to parse input as a number. Try again:");
                continue
            }
        };
    }    
}