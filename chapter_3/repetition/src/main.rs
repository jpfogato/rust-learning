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
                break 'counting_up; // <- this alias will make the outer loop to stop
            }
            remaining -= 1;
        }

        count +=1;
    }
    println!("End count = {count}");


}