fn main() {
        struct User {
                active: bool,
                username: String,
                email: String,
                sign_in_count: u64,
        }

        let mut user1 = User{
                // The entire instance must be mutable!
                email: String::from("someone@email.com"),
                username: String::from("username123"),
                // email and usernames are stored in the Heap as they are of type String
                active: true,
                sign_in_count: 1,
        };

        user1.email = String::from("updated_email@example.com");

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

}

