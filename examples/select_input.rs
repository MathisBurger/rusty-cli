use rusty_cli::inputs::select_input::SelectInput;

fn main() {

    let items = vec!["Option 1".to_string(), "Option 2".to_string()];

    let index = SelectInput::get_index(items.clone());
    println!("{}", index.unwrap());

    let value = SelectInput::get_value(items);
    println!("{}", value);
}