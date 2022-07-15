use dialoguer::Password;

pub struct PasswordInput {}

impl PasswordInput {

    /// Gets the value out of a password field
    pub fn get_value(prompt: Option<String>, reenter: bool) -> Option<String> {
        let mut input = Password::new();
        if prompt.is_some() {
            input.with_prompt(prompt.unwrap());
        }
        if reenter {
            input.with_confirmation("Enter again", "Passwords are not the same");
        }
        let interaction_result = input.interact();
        if interaction_result.is_err() {
            return None;
        }
        Some(interaction_result.unwrap())
    }
}