// Generates 50 random numbers between 1 and 30 and prints them to the console.
use rand::Rng;
use std::fs;

fn main() {
    let mut numbers = String::new();
    for _ in 0..50 {
        let number = generate_number();
        numbers.push_str(format!("{}\n", number).as_str());
    }
    create_or_write(&numbers);
}

fn generate_number() -> i32 {
    let mut random_number = rand::thread_rng();
    let number = random_number.gen_range(1..30);
    number
}

fn create_or_write(content: &String) {
    match fs::write("numbers.txt", content) {
        Ok(_) => println!("File created successfully"),
        Err(e) => println!("Error: {}", e),
    }
}
