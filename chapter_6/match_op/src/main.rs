#[derive(Debug)]
enum UsState{
    California,
    Texas,
    Florida,
    NewYork,
    // ...continues
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter (UsState), // enum variants can hold any type 
}

fn value_in_cents (coin: Coin) -> u8 {
    match coin { // evaluates the conditional expression, just like an if
        // however, in if, the expression allows only for a bool result
        // here it can be of any type <T>
        // patern => code to run
        // => is the operator
        Coin::Penny => {
            println!("Found a lucky penny!");
            1
        } // no comma is necessary if using curly brackets
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

/*
function that takes an Option<i32> and, if there’s a value inside, adds 1 to that value. If there isn’t a value inside, the function should return the None value and not attempt to perform any operations.
*/
fn plus_one (x: Option<i32>) -> Option<i32> {
    match x {
        None => None, //todo!("none handler"), 
        Some(i) => Some(i + 1),
    }
}

fn execute_something() {}
fn execute_something_else (){}
fn default_execution (){}
fn re_execute(){}

fn main() {
    value_in_cents(Coin::Quarter(UsState::California));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll_1 = 9;
    match dice_roll_1 {
        //the last pattern will match all values not specifically listed.
        //catch-all arm is last, because the operator checks all possible options in sequence
        3 => execute_something(),
        7 => execute_something_else(),
        other => default_execution(), // there's no need to declare this variable previously!
    }

    let dice_roll_2 = 9;
    match dice_roll_2 {
        // if we no longer need to use the catch-all value,
        // we can change our code to use _ instead of the variable named other
        3 => execute_something(),
        7 => execute_something_else(),
        _ => re_execute(),
    }

    let dice_roll_2 = 9;
    match dice_roll_2 {
        // now, nothing happens at the catch-all arm
        3 => execute_something(),
        7 => execute_something_else(),
        _ => (),
    }

}


