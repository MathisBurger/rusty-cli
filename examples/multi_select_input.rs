use rusty_cli::inputs::multi_select_input::MultiSelectInput;

fn main() {

    let items = vec!["Option 1".to_string(), "Option 2".to_string()];

    let indexes = MultiSelectInput::get_selected_indexes(items.clone());
    println!("{:?}", indexes);

    let values = MultiSelectInput::get_selected_values(items);
    println!("{:?}", values);
}