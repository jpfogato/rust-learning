use std::io; // Import the io module for input/output operations
use rand::Rng; // Import the Rng trait to use random number generation
use std::cmp::Ordering;

fn main() {
        println!("Guess the number!");

        let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
        // the guess variable will be shadowed later to u32, so we can use ":"
        // to define the type of the secret_number variable now

        // println!("The secret number is: {secret_number}");
        
        println!("Please input your guess.");

        loop{                
                let mut guess: String = String::new();
                // it is important to declare the variable here again because in the second time
                // the loop runs, the variable would have been shadowed to become an u32.
                // by doing this, we transform it back to a string so the user can bind a value to it.

                io::stdin() // Read user input
                        .read_line(&mut guess)
                        .expect("Failed to read line");
                
                let guess: u32 = match guess.trim()
                                // trim method also removes \r\n, not only \s
                                .parse(){
                                        // Ok(type to test for)                                       
                                        Ok(num) => num,
                                        // The underscore, _, is a catch-all value
                                        Err(_) => {
                                                println!("Please type a numeric value");
                                                continue; // finishes this loop iteration early
                                        }
                                };
                
                println!("You guessed: {guess}");  
                
                match guess.cmp(&secret_number){
                        Ordering::Less => println!("Too small!"),
                        Ordering::Greater => println!("Too big!"),
                        Ordering::Equal => {
                                println!("You win!");
                                break;
                        }
                }                
        }

}