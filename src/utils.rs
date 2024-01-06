// utils.rs

/// This is a private method used to generate the email address of the user.
/// It simply formats the `username` to lowercase, then appends `@example.com` to it.
pub fn form_email_id(username: &str) -> String {
    format!("{}@example.com", username.to_lowercase())
}
