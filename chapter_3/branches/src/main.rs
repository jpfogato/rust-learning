fn main(){
        let number = 7;
        if number < 5 {
                println!("condition was true");
        } else {
                println!("condition was false");
        }

        { // shadowing the number variable in this scope
                let number = 0;
                if number != 0 {
                        println!("number was something other than zero");
                }
        }

        {
                let number = 6;

                if number % 4 == 0 {
                        println!("number is divisible by 4");
                } else if number % 3 == 0 {
                        println!("number is divisible by 3");
                } else if number % 2 ==0 {
                        println!("number is divisible by 2");
                } else {
                    println!("number is not divisible by 4, 3 or 2");
                }
        }

        {    
                let condition = false;
                // Because if is an expression, we can use it in the right side of a statement to assign a value conditionally:
                let number = if condition { 1 } else { 0 };

                /*
                This would not compile as the "5" and "six" are of different types
                if condition { 5 } else { "six" };
                */ 
                println!("the value of number is: {number}");
        }
}