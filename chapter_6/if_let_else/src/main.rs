use std::intrinsics::fallback::FunnelShift;

fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (), // this is verbose. the implementation below solves that
    }

    // it only accounts for the value of Some being not Null
    // if it is possible to have something else other than it, it will return the
    // first value assignment
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}")
    }
}

#[derive(Debug)]
enum UsState {
    California,
    Texas,
    Florida,
    NewYork,
    // ...continues
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::California => year >= 1819,
            UsState::Texas => year >= 1715,
            UsState::Florida => year >= 1622,
            UsState::NewYork => year >= 1712,
            // continues
        }
    }
}

fn describe_state_quarter_1(coin: Coin) -> Option<String> {
    // a little hard to follow in more complex cases as the work
    // is done by the if let statement
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}

fn describe_state_quarter_2(coin: Coin) -> Option<String> {
    // returns early if not a UsState and returns a String if it is
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };

    if state.existed_in(1990) {
        Some(format!("{state:?} is pretty old!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn describe_state_quarter_3(coin: Coin) -> Option<String> {
    // uses let .. else to be less verbose
    let Coin::Quarter(state) = coin else {
        return None;
    };
    if state.existed_in(1990) {
        Some(format!("{state:?} is pretty old!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // enum variants can hold any type
}
