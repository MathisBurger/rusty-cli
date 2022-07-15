use dialoguer::console::Term;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use std::io::Result;

pub struct SelectInput {}

impl SelectInput {

    /// Gets the value of the selection action
    pub fn get_value(items: Vec<String>) -> String {
        items.get(SelectInput::get_selection(items.clone())
            .unwrap()
            .unwrap())
            .unwrap()
            .to_string()
    }

    /// Gets the index of the item selection
    pub fn get_index(items: Vec<String>) -> Option<usize> {
        SelectInput::get_selection(items).unwrap()
    }

    /// Gets the actual selection response
    fn get_selection(items: Vec<String>) -> Result<Option<usize>> {
        Select::with_theme(&ColorfulTheme::default())
            .items(&items)
            .default(0)
            .interact_on_opt(&Term::stderr())
    }
}

