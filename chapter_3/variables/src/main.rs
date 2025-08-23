fn main() {
        let mut x = 5;
        println!("The value of x is {x}");
        // x = 6 without declaring x with the mut keyword returns error https://doc.rust-lang.org/stable/error_codes/E0384.html 
        x = 6;
        println!("The value of x is now {x}");

        let y = 5;
        let y = y + 1;

        {
                let y = y + 2;
                println!("The value of y in the inner scope is: {y}");
        }

        println!("The original value of y (out of the scope) is: {y}");


        let z: u32 = 1;
        {
                let mut z = z;
                z += 2;
                // sintax highlighting will show that "z" is never actually used in our program inside this scope!
                // fix this by uncommenting the line:
                println!("z in the inner scope is: {z}");
        }
        // from the book quizz: answer is 1 for the value of z
        // this is because z is declared outside of this inner scope, which shadowed the variable in it

        println!("z's original value is: {z}");

        let a: f32 = 2.0;
        println!("{a}");

}
