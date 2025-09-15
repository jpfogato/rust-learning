fn main() {
        let mut counter = 0; // counter will change, so variable must be muttable!

        let result = loop {
                if counter == 10 {
                println!("counter value (true): {counter}");
                break counter * 2; // <- this returns the "counter * 2" value. If it wasn't here, the "result" variable has nothing to bind to and the program won't compile!
                } else {
                        println!("counter value (false): {counter}");
                }
                counter += 1;
        }; // <- this semicolon is important as this loop retunrs a value (thus becoming an expression)

        println!("the result is {result}");

        /*
        here are some examples of labeled loops. In conjunction with break and continue, it can be used to specify which loop should terminate.
        */

        /*
        The outer loop has the label 'counting_up, and it will count up from 0 to 2.
        The inner loop without a label counts down from 10 to 9.
        The first break that doesn’t specify a label will exit the inner loop only.
        The break 'counting_up; statement will exit the outer loop. 
        */

        let mut count = 0;

        'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
                println!("remaining = {remaining}");
                if remaining == 9 {
                        break
                }
                if count == 2 {
                        break 'counting_up; // <- this alias will make the outer loop to stop too
                }
                remaining -= 1;
        }

        count +=1;

        }

        println!("End count = {count}");

        {
                let mut number = 3;
                
                while number != 0 { // while number is not equal to zero
                        println!("{number}");
                        number -= 1;
                }

                println!("end of countdown");

                let a = [10, 20, 30, 40, 50];
                let mut index = 0;

                while index < 5 {  // if we try to read above index 4 the app will panic
                        println!("while -> the value of index {index} is: {}", a[index]);
                        index +=1;
                }

        }

        {
                let a = [10, 20, 30, 40, 50];

                let mut index = 0;

                for element in a {
                        println!("for -> the value of index {index} is: {element}");
                        index += 1;
                }
        }

        {
                for number in (1..4).rev() {
                        println!("number using Range + .rev(): {number}");
                }
        }

        {
                let a = [5; 10]; // Creates an array with 10 elements, all initialized to 5
                let mut sum = 0;
                for x in a {
                        sum += x;
                }
                println!("{sum}");
        }

}