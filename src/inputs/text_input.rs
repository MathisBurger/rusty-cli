use dialoguer::Input;

pub struct TextInput {}

impl TextInput {

    /// Gets the value of the text input
    pub fn get_value(prompt: Option<String>) -> String {
        let mut input = Input::<String>::new();
        if prompt.is_some() {
            input.with_prompt(prompt.unwrap());
        }
        input.interact().unwrap()
    }
}