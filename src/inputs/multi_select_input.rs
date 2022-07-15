use dialoguer::MultiSelect;

pub struct MultiSelectInput {}

impl MultiSelectInput {

    /// Gets all selected values from the input
    pub fn get_selected_values(items: Vec<String>) -> Vec<String> {
        let mut values = vec![];
        let indexes = MultiSelectInput::get_selected_indexes(items.clone());
        for index in indexes {
            values.push(items.get(index).unwrap().to_string());
        }
        values
    }

    /// Gets all selected indexes from the input
    pub fn get_selected_indexes(items: Vec<String>) -> Vec<usize> {
        MultiSelect::new()
            .items(&items)
            .interact()
            .unwrap()
    }
}