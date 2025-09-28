fn main() {
    struct User {
        /*
        we used the owned String type rather than the &str string slice type. This is a deliberate choice because we want each instance of this struct to own all of its data and for that data to be valid for as long as the entire struct is valid
        */
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let mut user1 = User {
        // The entire instance must be mutable!
        email: String::from("someone@email.com"),
        username: String::from("username123"),
        // email and usernames are stored in the Heap as they are of type String
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("updated_email@example.com");

    let user2 = User {
        email: String::from("another_email@example.com"),
        ..user1 // this notation allows us to set all othe values of this insnance with the same values from "user1" instance.
    };

    // after implementing user2, the value for username is owned by user2 instead of user1
    // if we try to use the user1's username, we get an an error:

    // user1.username; // <- enabling this generates a 'use of moved value' compiller error

    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username, // has the same name, so we can use shorthand notation
            // Replaces this code -> username: username,
            email,
            // Replaces this code -> email: email,
            sign_in_count: 1,
        }
    }

    let user3 = build_user(String::from("email"), String::from("username"));

    // Rust also supports tuple structs:
    // Use this to differentiate between tuples

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    black.1; // accesses index 1 of black

    let Point(x, y, z) = origin; // destructs the origin into x, y and z;

    struct AlwaysEqual; // this is a unity type struct. It is useful when the data that will compose it is still not known when developing

    let subject = AlwaysEqual;
}
