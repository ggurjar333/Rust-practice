// user.rs
use crate::utils;
pub struct User {
    /// The `username` field is a `String` that stores the username of the user.
    pub username: String,

    /// The `email` field is a `String` that stores the email of the user. This is automatically generated in the `new` function by appending `@example.com` to the lowercase of the username.
    pub email: String,
}

impl User {
    /// This is a constructor method for `User`.
    /// It takes a `&str` which represents the username of the new `User` to be created.
    /// It constructs and returns a new instance of `User`, initiating the `username` with the provided `username` and the `email` is generated using the `form_email_id` method.
    pub fn new(username: &str) -> User {
        User {
            username: username.to_string(),
            email: utils::form_email_id(username),
        }
    }

    /// This method allows for the updating of the username of the user.
    /// It takes a `&str which represents the new username.
    /// After updating the username, it also updates the `email` field accordingly.
    pub fn update(&mut self, new_username: &str) {
        self.username = new_username.to_string();
        self.email = utils::form_email_id(new_username);
    }

    /// This method greets a user by printing out a greeting message alongside the user's username and email address.
    pub fn greet(&self) {
        println!("Hello, {}!", self.username);
        println!("Your email is: {}", self.email);
    }
}
