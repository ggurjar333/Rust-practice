/// This module is responsible for handling user-related functionality.
/// It provides structs and functions for user management.
mod user;
mod utils;

use user::User;
/// # Main
///
/// The `main` function is the entry point of the program. It creates a new `User` object
/// and calls the `greet` method on it.
///
/// # Example
///
/// ```rust
/// fn main() {
///     let user = User::new("Gaurav");
///     user.greet();
/// }
/// ```
fn main() {
    let mut user = User::new("Gaurav");
    user.greet();
    println!("Let's update the name {}!", user.username);
    user.update("Gurjar");
    println!("After updating, it is {}!", user.username);
    user.greet();
}