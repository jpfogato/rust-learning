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

        // float declaration f32 or f64 (default)
        let a: f32 = 2.54;
        println!("float variable a = {a}");

        // tuple binding <-set to mut to allow modification
        let mut tuple: (i32, f64, u8, &str) = (-22, 6.4, 0xFF, "welcome");

        // destructuring the value of the tuple to the b, c, d and e variables.
        let (b, c, d, e) = tuple;
        println!("tuple variable: b = {b}, c = {c}, d = {d} and e = {e}");

        // it is also possible to index the value we want from the tuple and bind them to a new variable
        let i32_variable = tuple.0;
        let f64_variable = tuple.1;
        let u8_variable = tuple.2;
        let string_variable = tuple.3;
        println!("{i32_variable}, {f64_variable}, {u8_variable}, {string_variable}");

        // we can modify the elements of the tuple by:
        tuple.0 = 1;
        tuple.1 = 3.1416;
        tuple.2 = 0xAA;
        tuple.3 = "rust";

        let (b, c, d, e) = tuple;
        println!("b = {b}, c = {c}, d = {d} and e = {e}");

        // array type declaration
        let array = [5, 4, 3, 2, 1];
        let first = array[0];
        println!("array's first element: {first}");

        // or
        let array = [0;5]; // initializes an array of 5 elements filled with 0s
        let first = array[0];
        println!("array's first element is now: {first}");

        //let first = array[0]; 
        //let second = array[1];

        // from the quizz:
        let t = ([1; 2], [3; 4]);
        let (ta, tb) = t;
        println!("ta[0] + t1.[0] = {}", ta[0] + t.1[0]);
        println!("tb = {}", tb[0]);

}
