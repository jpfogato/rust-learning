fn main() {
        println!("Hello, world!");
        another_function();
        function_with_parameter(42);
        print_labeled_measurement(2.1456, 'm');
        expression();
        let x = five();
        println!("Return function: {}", x);
        let x_incremented = increment(x);
        println!("Incremented x is: {}", x_incremented);
        quizz();

}

fn another_function(){
// could have been defined prior to the main
        println!("Another function!");
}

fn function_with_parameter(x: i32){
        println!("The value of x is {}", x);
}

fn print_labeled_measurement(value: f64, unit_label: char){
        println!("The measurement is: {} {}", value, unit_label);
}

fn expression(){
        /*
        as the x + 1 is an expression, it returns a value that can
        be bind to y, so we do not have an error here
         */
        let y = {
                let x = 3;
                x + 1
                /*
                Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value
                 */
        };
        println!("Expression value is: {}", y);
}

fn five() -> i32{
        5
}

fn increment(x: i32) -> i32{
        x + 1 // if a ; is added here, we will get E0308: mismatched types
}

/*
        this is a statement as it does not return a value:
                let x = 10;

        As this operation does not return a variable, this will cause an error:
                let y = (let x = 10);
        error: expected expression, found `let` statement

        The let x = 10 statement does not return a value, so there isn’t anything for x to bind to.

*/


// From the quizz:


fn f(x: i32) -> i32 { x + 1 }

fn quizz() {
  println!("Quizz' returned value is: {}", f({
    let y = 1;
    y + 1
  }));
}