use std::fmt::Result;
use std::io::Result as IoResult; // must be specified as both io and fmt brings into scope a
                                 // "Result" type

// returns the "Result" from std::fmt
fn function1() -> Result {
    Ok(())
}

// returns the "Result" from std::io::Result >>as<< IoResult
fn function2() -> IoResult<()> {
    Ok(())
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
