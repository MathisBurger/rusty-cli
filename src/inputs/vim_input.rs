use dialoguer::Editor;

pub struct VimInput {}

impl VimInput {

    /// Gets the edited message back
    pub fn get_value(message_to_edit: String) -> Option<String> {
        Editor::new().edit(message_to_edit.as_str()).unwrap()
    }
}