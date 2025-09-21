/*
Prompt the user for a number n.
Handle the base cases (if n is 0 or 1).
For any larger n, use a loop to calculate the value iteratively.
You'll need to track two previous numbers (e.g., a and b).
In each loop iteration, calculate the next number and update your tracked values.
*/
use std::io;

fn main() {
        println!("Fibonacci number calculator\nType a sequence number \"n\" to discover it's equivalent in the Fibonacci sequence\nWhere f_n = F_n-1 + F_n-2\n\nWrite a number to get its sequence:");
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("invalid input");
        let input = input.trim();
        let n: u32 = match input.parse(){
                        Ok(n) => n,
                        Err(_) => {
                                println!("Value is not a number");
                                0 // returns 0 from parser.
                        }
                };
        let f_n = fibonacci(n);
        println!("Fibonacci value is: {f_n}");

}

fn fibonacci(n: u32) -> u32 {
        if n == 0 {
                return 0;
        } else if n == 1 {
                return 1;
        }
    
        let mut a = 0;
        let mut b = 1;

        for _ in 2..=n {
                let next = a + b;
                a = b;
                b = next;
        }
        b
}