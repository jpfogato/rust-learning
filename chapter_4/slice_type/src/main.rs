fn main() {
    /*
    Small programming problem: write a function that takes a string of words separated by spaces and returns the first word it finds in that string
    */

    const STRING_CONSTANT: &str = "This is my sample string";

    // String literals are slices!

    let s = "Hello, world!"; // This string literal is already a slice!

    first_word_iteractive(&STRING_CONSTANT.to_string());
    first_word_sliced(&STRING_CONSTANT.to_string());
    first_word_literal(STRING_CONSTANT); // no need to convert!

    let mut s = String::from("Hello");
    let hello: &str = &s[0..5]; // s loses W and O permissions as hello is now pointing to it
    println!("{hello}"); // s now regains W and O permissions as hello is dropped after it has been used

    s.push_str(" world");
}

fn first_word_iteractive(s: &String) -> usize {
    let bytes = s.as_bytes(); // wee need to iterate on the string, so this convertion is done to facilitate that

    for (i, &item) in bytes.iter().enumerate() {
        // iter: iterator over the array of bytes
        // enumerate: wraps the result of iter and returns each element as part of tuple instead formated as "(index, ref to the element)"

        if item == b' ' {
            // b'...' is a byte-literal syntax
            return i;
            // if the byte equivalent of the ' '
            // if a space is found, "i" (index) is returned
        }
    }

    s.len()
}

fn first_word_sliced(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..] // notice the different return type!
}

fn first_word_literal(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut index = 0;
    for &item in bytes {
        if item == b' ' {
            return &s[0..index];
        }
        index += index;
    }
    &s[..]
}
