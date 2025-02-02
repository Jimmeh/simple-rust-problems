use std::io;

fn main() {
    println!("Program that draws triangles to the console.");
    let draw_fn = prompt_for_type();
    let length = prompt_for_length();
    draw_fn(length);
}

fn draw_base_at_top(length: usize) {
    for i in (1..length+1).rev() {
        println!("{}", "*".repeat(i))
    }
}

fn draw_even_steps_skipped_base_at_bottom(length: usize) {
    for i in 1..length+1 {
        if i % 2 != 0 {
            println!("{}", "*".repeat(i))
        }
    }
}

fn draw_base_at_bottom(length: usize) {
    for i in 1..length+1 {
        println!("{}", "*".repeat(i))
    }
}

fn prompt_for_type() -> fn(usize) {
    loop {
        println!("Select triangle type:");
        println!("1: Base at top");
        println!("2: Base at bottom");
        println!("3: Even steps, base at bottom");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        match choice.trim() {
            "1" => return draw_base_at_top,
            "2" => return draw_base_at_bottom,
            "3" => return draw_even_steps_skipped_base_at_bottom,
            _ => continue
        }
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