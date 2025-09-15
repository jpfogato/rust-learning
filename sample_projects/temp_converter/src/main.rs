/* Description: A command-line program that converts a temperature from Fahrenheit to Celsius and vice-versa.
This project is great for practicing functions, variable binding, and basic arithmetic. */

use std::io; // input/output operations

fn c_to_f (temp_c: f64) -> f64 {
        // convert from C to F
        let temp_f: f64 = temp_c * 9.0/5.0 + 32.0;
        return temp_f;
}

fn f_to_c (temp_f: f64) -> f64 {
        // convert from F to C
        let temp_c: f64 = (temp_f - 32.0) / 1.8;
        return temp_c;
}

fn convert_temperature (option: u32, temp_input: f64) -> f64 {
        // run the convertion function selected using option
        let converted_temperature = 
                match option {
                        1 => c_to_f(temp_input),
                        2 => f_to_c(temp_input),
                        _ => panic!("invalid option")
                };
        return converted_temperature;
}

fn main() {
        println!("Temperature Conversion App");

        loop {
                println!("type '1' for C -> F or '2' for F -> C or 'exit' to finish");              
                
                let mut input: String = String::new();
                
                io::stdin()
                        .read_line(&mut input)
                        .expect("invalid input");

                let trimmed_input = input.trim(); // Trim once and reuse

                if trimmed_input == "exit" {
                        break;
                }

                let option: u32 = match trimmed_input
                        .parse(){
                                Ok(num) => num,
                                Err(_) => {
                                                println!("Please type an option value as a numeric");
                                                continue;
                                        }
                        };

                match option {
                        1 | 2 => {}
                        _ => { // anything else
                                println!("invalid option, try again...");
                                continue;
                        }
                };

                let mut temp_input = String::new();

                let temp_string = if option == 1 {"C"} else {"F"};
                                    
                println!("type the temperature in degrees {temp_string}: ");

                io::stdin()
                        .read_line(&mut temp_input)
                        .expect("failed to read input value");
                
                let temp_input: f64 = match temp_input
                        .trim()
                        .parse(){
                                Ok(num) => num,
                                Err(_) => {
                                        if temp_input == "exit" {
                                                break;
                                        } else {
                                                        println!("Please type a valid numeric value.");
                                                        continue;
                                                }
                                        }
                                };


                let converted_temperature = convert_temperature(option, temp_input);
                println!("converted temperature: {converted_temperature} degrees {temp_string}");
        }                  
}
                