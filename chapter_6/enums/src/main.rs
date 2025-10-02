fn main() {
    enum IpAddressKind {
        V4(u8, u8, u8, u8), // rather than an enum inside a struct, we can put data directly into each enum variant
        V6(String),         // this way, we attach data to each variant of the enum directly
                            // each variant can have different types and amounts of associated data
    }

    let home = IpAddressKind::V4(127, 0, 0, 1);
    let loopback = IpAddressKind::V6(String::from("::1"));

    enum Message {
        Quit,                       // has no data associated with it
        Move { x: i32, y: i32 },    // has named fields, like a struct...
        Write(String),              // includes a single string
        ChangeColor(i32, i32, i32), // includes three i32 values
    }

    impl Message {
        // It is possible to implement methods for enums too!
        fn call(&self) {}
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    // The Option type encodes the very common scenario in which a value could be something or it could be nothing

    let some_number = Some(5);
    let some_char = Some('e');
    
    // The compiler can’t infer the type that the corresponding Some variant will hold by looking only at a None value.
    // Here, we tell Rust that we mean for absent_number to be of type Option<i32>.
    let absent_number: Option<i32> = None;
    
}
