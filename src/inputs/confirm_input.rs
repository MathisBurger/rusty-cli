use dialoguer::Confirm;

pub struct ConfirmInput {}

impl ConfirmInput {

    /// Gets the value of the confirm input
    pub fn get_value(text: String) -> bool {
        Confirm::new().with_prompt(text).interact().unwrap()
    }
}