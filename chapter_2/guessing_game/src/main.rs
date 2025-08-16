use std::io; // Import the io module for input/output operations
use rand::Rng; // Import the Rng trait to use random number generation

fn main() {
        println!("Guess the number!");
        println!("Please input your guess.");

        let mut guess = String::new();

        // Generate a random number between 1 and 100
        let secret_number = rand::thread_rng().gen_range(1..=100);
        println!("The secret number is: {secret_number}");
        
        // Read user input
        io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

        println!("You guessed: {guess}");  
}